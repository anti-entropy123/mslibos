//! `elf_service` use POSIX API `dlopen` to create service.
//! In the future, it will be **discarded**.

use std::{
    collections::{BTreeMap, HashSet},
    ffi::c_void,
    mem::{forget, MaybeUninit},
    sync::Arc,
};

use lazy_static::lazy_static;
use libloading::{Library, Symbol};

use log::{error, info};
use ms_hostcall::{
    types::{DropHandlerFunc, IsolationID, MetricEvent, ServiceName},
    IsolationContext, SERVICE_HEAP_SIZE,
};
use nix::libc::RTLD_DI_LMID;

use crate::{
    isolation::handler::{find_host_call, panic_handler},
    logger,
    metric::SvcMetricBucket,
    GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol,
};

use super::loader::Namespace;

lazy_static! {
    static ref SHOULD_NOT_SET_CONTEXT: Arc<HashSet<ServiceName>> = Arc::from({
        let mut hs = HashSet::new();
        #[cfg(feature = "namespace")]
        {
            hs.insert("libc".to_owned());
        }
        hs.insert("stdio".to_owned());
        hs.insert("time".to_owned());
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
    heap: Box<MaybeUninit<ServiceHeap>>,
    metric: Arc<SvcMetricBucket>,
}

impl ELFService {
    pub fn new(name: &str, lib: Arc<Library>, metric: Arc<SvcMetricBucket>) -> Self {
        metric.mark(MetricEvent::SvcInit);
        logger::debug!("ELFService::new, name={name}");

        Self {
            name: name.to_string(),
            lib,
            heap: Box::new_uninit(),
            metric,
        }
    }

    fn should_set_context(&self) -> bool {
        !SHOULD_NOT_SET_CONTEXT.contains(&self.name.to_owned())
    }

    pub fn init(&self, isol_id: IsolationID) {
        let heap_start = self.heap.as_ptr() as usize;
        let heap_range = (heap_start, heap_start + SERVICE_HEAP_SIZE);
        logger::debug!(
            "init for service_{}, isol_id={}, find_host_call_addr=0x{:x}, heap_range={:x?}",
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

        let set_handler: SetHandlerFuncSybmol = self
            .symbol("set_handler_addr")
            .expect("missing set_handler_addr?");
        logger::info!("start set_handler...");
        unsafe { set_handler(&isol_ctx) }.expect("service init failed.");
        logger::info!("set_handler complete.");

        let get_handler: GetHandlerFuncSybmol = self
            .symbol("get_handler_addr")
            .expect("missing get_handler_addr?");
        logger::debug!(
            "service_{} get_hander addr=0x{:x}.",
            self.name,
            *get_handler as usize
        );
        assert!(unsafe { get_handler() } == find_host_call as usize)
    }

    pub fn symbol<T>(&self, symbol: &str) -> Option<Symbol<T>> {
        unsafe { self.lib.get(symbol.as_bytes()) }.ok()
    }

    pub fn run(&self, args: &BTreeMap<String, String>) -> Result<(), ()> {
        let rust_main: RustMainFuncSybmol = self.symbol("rust_main").expect("missing rust_main?");
        log::info!(
            "service_{} rust_main={:x} thread_name={}",
            self.name,
            (*rust_main) as usize,
            std::thread::current().name().unwrap()
        );

        self.metric.mark(MetricEvent::SvcRun);
        let result = unsafe { rust_main(args) };
        self.metric.mark(MetricEvent::SvcEnd);

        logger::info!("{} complete.", self.name);
        if let Err(e) = result {
            error!("run failed: {}", e);
            // forget because String refer to heap of libos modules.
            forget(e);
        }
        Ok(())
    }

    pub fn namespace(&self) -> Namespace {
        // The reason for using this hack, is same to `fn load_dynlib()`, that must
        // get `handle: *mut c_void` to call `dlinfo()`.
        let handle: usize = *unsafe { core::mem::transmute::<&Library, &usize>(self.lib.as_ref()) };
        let mut result: Namespace = Namespace::default();

        let info = &mut result as *mut Namespace as usize;
        unsafe { nix::libc::dlinfo(handle as *mut c_void, RTLD_DI_LMID, info as *mut c_void) };
        info!("service_{} belong to namespace: {}", self.name, result);
        result
    }
}

impl Drop for ELFService {
    fn drop(&mut self) {
        if let Some(drop_fn) = self.symbol::<DropHandlerFunc>("drop") {
            logger::info!("service {} will invoke drop symbol.", self.name);
            unsafe { drop_fn() }
        }
    }
}
