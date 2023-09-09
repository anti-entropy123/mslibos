//! `elf_service` use POSIX API `dlopen` to create service.
//! In the future, it will be **discarded**.

use std::{
    borrow::Cow,
    collections::HashSet,
    ffi::{CStr, CString},
    os::{
        raw::{self, c_void},
        unix::prelude::OsStrExt,
    },
    path::PathBuf,
    sync::Arc,
};

use anyhow::anyhow;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

use log::info;
use ms_hostcall::{
    types::{DropHandlerFunc, IsolationID, ServiceName},
    IsolationContext, SERVICE_HEAP_SIZE,
};
use nix::libc::{
    dlerror, dlinfo, dlmopen, Lmid_t, LM_ID_NEWLM, RTLD_DI_LMID, RTLD_LAZY, RTLD_LOCAL,
};

use crate::{
    isolation::handler::{find_host_call, panic_handler},
    logger,
    metric::{MetricEvent, SvcMetricBucket},
    utils, GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol,
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
    heap: Arc<ServiceHeap>,
    metric: Arc<SvcMetricBucket>,
}

impl ELFService {
    pub fn new(
        name: &str,
        filename: &PathBuf,
        namespace: Option<&Namespace>,
        metric: Arc<SvcMetricBucket>,
    ) -> Self {
        metric.mark(MetricEvent::SvcInit);
        let lib = Arc::from(
            load_dynlib(filename, namespace.map(|ns| ns.as_lmid_t())).expect("failed load dynlib"),
        );
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

        let set_handler: SetHandlerFuncSybmol = self
            .symbol("set_handler_addr")
            .expect("missing set_handler_addr?");
        logger::info!("start set_handler...");
        unsafe { set_handler(isol_ctx) }.expect("service init failed.");
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

    pub fn run(&self) -> Result<(), ()> {
        let rust_main: RustMainFuncSybmol = self.symbol("rust_main").expect("missing rust_main?");
        log::info!(
            "service_{} rust_main={:x}",
            self.name,
            (*rust_main) as usize
        );

        self.metric.mark(MetricEvent::SvcRun);
        let result = unsafe { rust_main() };
        self.metric.mark(MetricEvent::SvcEnd);

        logger::info!("{} complete.", self.name);
        result
    }

    pub fn namespace(&self) -> Namespace {
        // The reason for using this hack, is same to `fn load_dynlib()`, that must
        // get `handle: *mut c_void` to call `dlinfo()`.
        let handle: usize = *unsafe { core::mem::transmute::<&Library, &usize>(self.lib.as_ref()) };
        let mut result: Namespace = Namespace::default();

        let info = &mut result as *mut Namespace as usize;
        unsafe { dlinfo(handle as *mut c_void, RTLD_DI_LMID, info as *mut c_void) };
        info!("service_{} belong to namespace: {}", self.name, result);
        result
    }
}

impl Drop for ELFService {
    fn drop(&mut self) {
        // match self.symbol::<DropHandlerFunc>("drop") {
        //     Some(drop_fn) => unsafe {
        //         logger::info!("service {} will invoke drop symbol.", self.name);
        //         drop_fn()
        //     },
        //     None => {}
        // };
        if let Some(drop_fn) = self.symbol::<DropHandlerFunc>("drop") {
            logger::info!("service {} will invoke drop symbol.", self.name);
            unsafe { drop_fn() }
        }
    }
}

#[test]
fn service_drop_test() {
    use crate::{metric::MetricBucket, service::elf_service::ELFService};
    // std::env::set_var("RUST_LOG", "INFO");
    // logger::init();

    let bucket = MetricBucket::new();
    let path = PathBuf::new().join("target/debug/libsocket.so");

    let socket = ELFService::new(
        "socket",
        &path,
        None,
        bucket.new_svc_metric("socket".to_owned()),
    );

    drop(socket)
}

/// Checks for the last byte and avoids allocating if it is zero.
///
/// Non-last null bytes still result in an error.
pub(crate) fn cstr_cow_from_bytes(slice: &[u8]) -> anyhow::Result<Cow<'_, CStr>> {
    static ZERO: raw::c_char = 0;
    Ok(match slice.last() {
        // Slice out of 0 elements
        None => unsafe { Cow::Borrowed(CStr::from_ptr(&ZERO)) },
        // Slice with trailing 0
        Some(&0) => Cow::Borrowed(CStr::from_bytes_with_nul(slice)?),
        // Slice with no trailing 0
        Some(_) => Cow::Owned(CString::new(slice)?),
    })
}

#[cfg(feature = "namespace")]
fn do_dlmopen(filename: &std::path::Path, lmid: Option<Lmid_t>) -> anyhow::Result<*mut c_void> {
    let handle = unsafe {
        dlmopen(
            lmid.unwrap_or(LM_ID_NEWLM),
            cstr_cow_from_bytes(filename.as_os_str().as_bytes())?.as_ptr(),
            RTLD_LAZY | RTLD_LOCAL,
        )
    };

    info!("load_dynlib: dlmopen handle=0x{:x}", handle as usize);
    if handle.is_null() {
        let error = unsafe { dlerror() };
        return if error.is_null() {
            Err(anyhow!("unknown dlmopen error"))
        } else {
            let message = unsafe { CStr::from_ptr(error) }
                .to_string_lossy()
                .to_string();
            Err(anyhow!(message))
        };
    };
    Ok(handle)
}

fn load_dynlib(filename: &PathBuf, lmid: Option<Lmid_t>) -> anyhow::Result<Library> {
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
    let lib = {
        #[cfg(feature = "namespace")]
        // libloading do not supply any method like `from_raw(handle: *mut c_void)`.
        unsafe {
            core::mem::transmute(do_dlmopen(&filename, lmid)?)
        }

        #[cfg(not(feature = "namespace"))]
        {
            debug!("do not use dlmopen, lmid={:?} is meaningless", lmid);
            unsafe { Library::new(filename) }?
        }
    };
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
