use std::{path::PathBuf, sync::Arc};

use libloading::{Library, Symbol};

use crate::{logger, RustMainFuncSybmol, SetHandlerFuncSybmol};

pub struct Service {
    name: String,
    lib: Arc<Library>,
}

impl Service {
    pub fn new(name: &str, filename: PathBuf) -> Self {
        let lib = Arc::from(load_dynlib(filename).expect("failed load dynlib"));
        Self {
            name: name.to_string(),
            lib,
        }
    }

    pub fn run(&self) {
        let set_handler: SetHandlerFuncSybmol = find_symbol(&self.lib, "set_handler_addr");
        let rust_main: RustMainFuncSybmol = find_symbol(&self.lib, "rust_main");

        let find_host_call_addr = crate::find_host_call as usize;
        logger::debug!("find_host_call_addr = 0x{:x}", find_host_call_addr);

        unsafe {
            set_handler(find_host_call_addr).expect("set hostcall addr failed");
            rust_main()
        }
        logger::info!("{} complete.", self.name);
    }
}

pub fn load_dynlib(filename: PathBuf) -> anyhow::Result<Library> {
    let lib = unsafe { Library::new(filename) }?;
    anyhow::Ok(lib)
}

pub fn find_symbol<'a, T>(lib: &'a Library, symbol: &str) -> Symbol<'a, T> {
    unsafe { lib.get(symbol.as_bytes()).expect("find symbol failed") }
}
