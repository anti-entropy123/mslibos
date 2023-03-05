use std::{path::PathBuf, thread};

use libloading::Library;
use ms_hostcall::IsolationContext;
use msvisor::{
    logger,
    service::{find_symbol, load_dynlib},
    GetHandlerFuncSybmol, RustMainFuncSybmol, SetHandlerFuncSybmol,
};

fn run_hello_world(addr: usize) {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("debug")
        .join("libhello_world.so");
    let lib = load_dynlib(&filename).expect("failed load dynlib");
    let set_handler: SetHandlerFuncSybmol = find_symbol(&lib, "set_handler_addr");
    let get_handler: GetHandlerFuncSybmol = find_symbol(&lib, "get_handler_addr");
    let rust_main: RustMainFuncSybmol = find_symbol(&lib, "rust_main");

    logger::debug!("find_host_call_addr = 0x{:x}", addr);

    unsafe {
        set_handler(IsolationContext {
            isol_id: 1,
            find_handler: dummy_find_hostcall as usize,
        })
        .expect("set hostcall addr failed");
        assert!(get_handler() == addr);
        rust_main()
    }
}

unsafe extern "C" fn dummy_write() {
    return;
}

unsafe extern "C" fn dummy_find_hostcall() -> usize {
    dummy_write as usize
}

#[test]
fn run_dylib_test() {
    run_hello_world(dummy_find_hostcall as usize)
}

#[test]
#[should_panic]
fn repeat_set_handler_test() {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("debug")
        .join("libhello_world.so");
    let lib = unsafe { Library::new(filename).expect("load dyn failed") };
    let set_handler: SetHandlerFuncSybmol = unsafe {
        lib.get("set_handler_addr".as_bytes())
            .expect("find symbol failed")
    };
    unsafe {
        set_handler(IsolationContext {
            isol_id: 1,
            find_handler: 0x40000,
        })
        .expect("set hostcall addr failed");
        set_handler(IsolationContext {
            isol_id: 1,
            find_handler: 0x80000,
        })
        .expect("set hostcall addr failed");
    }
}

#[test]
fn run_multi_dylib_test() {
    for _ in 0..30 {
        thread::spawn(|| run_hello_world(dummy_find_hostcall as usize))
            .join()
            .expect("join thread failed");
    }
}
