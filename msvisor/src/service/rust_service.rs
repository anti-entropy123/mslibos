use std::{path::PathBuf, sync::Arc};

use libloading::{Library, Symbol};
use ms_hostcall::{types::IsolationID, IsolationContext, SERVICE_HEAP_SIZE};

use crate::{
    isolation::find_host_call, logger, GetHandlerFuncSybmol, RustMainFuncSybmol,
    SetHandlerFuncSybmol,
};

#[repr(C, align(4096))]
struct ServiceHeap {
    heap: [u8; SERVICE_HEAP_SIZE],
}

pub struct RustService {
    name: String,
    lib: Arc<Library>,
    heap: Arc<ServiceHeap>,
}

impl RustService {
    pub fn new(name: &str, filename: &PathBuf) -> Self {
        let lib = Arc::from(load_dynlib(filename).expect("failed load dynlib"));
        Self {
            name: name.to_string(),
            lib,
            heap: Arc::new(ServiceHeap {
                heap: [0; SERVICE_HEAP_SIZE],
            }),
        }
    }

    pub fn init(&self, isol_id: IsolationID) {
        let heap_start = self.heap.heap.as_ptr() as usize;
        let heap_range = (heap_start, heap_start + SERVICE_HEAP_SIZE);
        logger::debug!(
            "init for service_{}, isol_id = {}, find_host_call_addr = 0x{:x}, heap_range = {:x?}",
            self.name,
            isol_id,
            find_host_call as usize,
            heap_range
        );

        if self.name.contains("fs") {
            return
        };

        let isol_ctx = IsolationContext {
            isol_id,
            find_handler: find_host_call as usize,
            heap_range,
        };
        
        let set_handler: SetHandlerFuncSybmol = self.symbol("set_handler_addr");
        logger::info!("start set_handler...");
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
