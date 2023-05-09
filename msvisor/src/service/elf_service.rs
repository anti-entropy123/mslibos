//! `elf_service` use POSIX API `dlopen` to create service.
//! In the future, it will be **discarded**.

use std::{collections::HashSet, path::PathBuf, sync::Arc};

use anyhow::anyhow;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

use ms_hostcall::{
    types::{IsolationID, ServiceName},
    IsolationContext, SERVICE_HEAP_SIZE,
};

use crate::{
    isolation::handler::{find_host_call, panic_handler},
    logger,
    metric::{MetricEvent, SvcMetricBucket},
    utils, GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol,
};

lazy_static! {
    static ref SHOULD_NOT_SET_CONTEXT: Arc<HashSet<ServiceName>> = Arc::from({
        let mut hs = HashSet::new();
        hs.insert("stdio".to_owned());
        hs.insert("socket".to_owned());
        hs
    });
}

#[test]
fn test_should_not_set_context() {
    assert!(
        SHOULD_NOT_SET_CONTEXT.contains("stdio"),
        "SHOULD_NOT_SET_CONTEXT do not have 'stdio'"
    )
}

#[repr(C, align(4096))]
struct ServiceHeap {
    heap: [u8; SERVICE_HEAP_SIZE],
}

pub struct ELFService {
    pub name: String,
    lib: Arc<Library>,
    heap: Arc<ServiceHeap>,
    metric: Arc<SvcMetricBucket>,
}

impl ELFService {
    pub fn new(name: &str, filename: &PathBuf, metric: Arc<SvcMetricBucket>) -> Self {
        metric.mark(MetricEvent::SvcInit);
        let lib = Arc::from(load_dynlib(filename).expect("failed load dynlib"));
        logger::debug!("ELFService::new, name={name}");
        Self {
            name: name.to_string(),
            lib,
            heap: Arc::new(ServiceHeap {
                heap: [0; SERVICE_HEAP_SIZE],
            }),
            metric,
        }
    }

    fn should_set_context(&self) -> bool {
        !SHOULD_NOT_SET_CONTEXT.contains(&self.name.to_owned())
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

        // If this is a common_service that does not dependent on IsolationContext,
        // then directly return. Because it is not a no_std elf, and not have
        // symbols `set_handler_addr` and `get_handler_addr`.
        if !self.should_set_context() {
            return;
        };

        let isol_ctx = IsolationContext {
            isol_id,
            find_handler: find_host_call as usize,
            panic_handler: panic_handler as usize,
            heap_range,
        };

        let set_handler: SetHandlerFuncSybmol = self.symbol("set_handler_addr");
        logger::info!("start set_handler...");
        unsafe { set_handler(isol_ctx) }.expect("service init failed.");
        logger::info!("set_handler complete.");

        let get_handler: GetHandlerFuncSybmol = self.symbol("get_handler_addr");
        logger::debug!(
            "service_{} get_hander addr=0x{:x}.",
            self.name,
            *get_handler as usize
        );
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

        self.metric.mark(MetricEvent::SvcRun);
        unsafe { rust_main() };
        self.metric.mark(MetricEvent::SvcEnd);

        logger::info!("{} complete.", self.name);
    }
}

fn load_dynlib(filename: &PathBuf) -> anyhow::Result<Library> {
    let filename = if !filename.is_file() {
        utils::REPOS_ROOT_PATH.join(filename)
    } else {
        filename.to_owned()
    };

    if !filename.is_file() {
        return Err(anyhow!(
            "load dynlib failed. filename is invaild: {}",
            filename.to_str().unwrap()
        ));
    }

    let lib = unsafe { Library::new(filename) }?;
    anyhow::Ok(lib)
}

// This test case does not work for now, because dlmopen have not be used
// in service loader.
//
// #[test]
// fn test_load_dynlib() {
//     const TARGET_DIR: &str = concat!(
//         env!("CARGO_MANIFEST_DIR"),
//         "/../target/debug/libhello_world.so"
//     );
//     let filename = PathBuf::from(TARGET_DIR);
//     let lib1 = load_dynlib(&filename).unwrap();
//     let lib2 = load_dynlib(&filename).unwrap();
//     unsafe {
//         let addr1 = (*lib1.get::<fn()>(b"rust_main").unwrap()) as usize;
//         let addr2 = (*lib2.get::<fn()>(b"rust_main").unwrap()) as usize;

//         assert!(addr1 != addr2, "addr1:{:x} == addr2:{:x}", addr1, addr2);
//     }
// }
