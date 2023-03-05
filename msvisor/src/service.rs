use std::{collections::HashMap, path::PathBuf, sync::Arc};

use libloading::{Library, Symbol};
use ms_hostcall::{types::ServiceName, IsolationContext};

use crate::{
    isolation::find_host_call, logger, GetHandlerFuncSybmol, RustMainFuncSybmol,
    SetHandlerFuncSybmol,
};

pub struct ServiceLoader {
    registered: HashMap<ServiceName, PathBuf>,
}

impl ServiceLoader {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
    }

    pub fn register(mut self, service: ServiceName, lib_path: PathBuf) -> Self {
        self.registered.insert(service, lib_path);
        self
    }

    pub fn load_service(&self, isol_ctx: IsolationContext, name: &ServiceName) -> Arc<Service> {
        let lib_path = self.registered.get(name).expect("unregistered library!");

        let service = Service::new(name, lib_path);
        service.init(isol_ctx);
        Arc::from(service)
    }
}

impl Default for ServiceLoader {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Service {
    name: String,
    lib: Arc<Library>,
}

impl Service {
    fn new(name: &str, filename: &PathBuf) -> Self {
        let lib = Arc::from(load_dynlib(filename).expect("failed load dynlib"));
        Self {
            name: name.to_string(),
            lib,
        }
    }

    fn init(&self, isol_ctx: IsolationContext) {
        logger::debug!(
            "find_host_call_addr = 0x{:x}",
            isol_ctx.find_handler as usize
        );
        let set_handler: SetHandlerFuncSybmol = self.symbol("set_handler_addr");
        unsafe { set_handler(isol_ctx) }.expect("service init failed.");
        logger::info!("set_handler complete.");

        let get_handler: GetHandlerFuncSybmol = self.symbol("get_handler_addr");
        assert!(unsafe { get_handler() } == find_host_call as usize)
    }

    pub fn symbol<T>(&self, symbol: &str) -> Symbol<T> {
        unsafe { self.lib.get(symbol.as_bytes()) }.expect("find symbol failed")
    }

    pub fn run(&self) {
        let rust_main: RustMainFuncSybmol = self.symbol("rust_main");
        log::info!(
            "service_{} rust_main={:x}",
            self.name,
            (*rust_main) as usize
        );

        unsafe { rust_main() };

        logger::info!("{} complete.", self.name);
    }
}

pub fn load_dynlib(filename: &PathBuf) -> anyhow::Result<Library> {
    let lib = unsafe { Library::new(filename) }?;
    anyhow::Ok(lib)
}

pub fn find_symbol<'a, T>(lib: &'a Library, symbol: &str) -> Symbol<'a, T> {
    unsafe { lib.get(symbol.as_bytes()).expect("find symbol failed") }
}
