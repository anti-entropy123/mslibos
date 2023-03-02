use std::{path::PathBuf, thread};

use libloading::{Library, Symbol};
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
    let lib = load_dynlib(filename).expect("failed load dynlib");
    let set_handler: SetHandlerFuncSybmol = find_symbol(&lib, "set_handler_addr");
    let get_handler: GetHandlerFuncSybmol = find_symbol(&lib, "get_handler_addr");
    let rust_main: RustMainFuncSybmol = find_symbol(&lib, "rust_main");

    logger::debug!("find_host_call_addr = 0x{:x}", addr);

    unsafe {
        set_handler(addr).expect("set hostcall addr failed");
        assert!(get_handler() == msvisor::find_host_call as usize);
        rust_main()
    }
}

#[test]
fn run_dylib_test() {
    run_hello_world(msvisor::find_host_call as usize)
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
    let set_handler: Symbol<unsafe extern "C" fn(usize) -> Result<(), ()>> = unsafe {
        lib.get("set_handler_addr".as_bytes())
            .expect("find symbol failed")
    };
    unsafe {
        set_handler(0x40000).expect("set hostcall addr failed");
        set_handler(0x80000).expect("set hostcall addr failed");
    }
}

#[test]
fn run_two_dylib_test() {
    let t1 = thread::spawn(|| run_hello_world(msvisor::find_host_call as usize));
    let t2 = thread::spawn(|| run_hello_world(msvisor::find_host_call as usize));
    t1.join().expect("Couldn't join on the associated thread 1");
    t2.join().expect("Couldn't join on the associated thread 2");
}
