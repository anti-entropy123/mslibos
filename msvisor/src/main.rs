mod logger;

use std::path::PathBuf;

use msvisor::{find_symbol, load_dynlib, GetHandlerFunc, RustMainFunc, SetHandlerFunc};

fn main() {
    logger::init();

    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("debug")
        .join("libhello_world.so");
    let lib = load_dynlib(filename).expect("failed load dynlib");
    let set_handler: SetHandlerFunc = find_symbol(&lib, "set_handler_addr");
    let get_handler: GetHandlerFunc = find_symbol(&lib, "get_handler_addr");
    let rust_main: RustMainFunc = find_symbol(&lib, "rust_main");

    let find_host_call_addr = msvisor::find_host_call as usize;
    logger::debug!("find_host_call_addr = 0x{:x}", find_host_call_addr);

    unsafe {
        set_handler(find_host_call_addr).expect("set hostcall addr failed");
        assert!(get_handler() == msvisor::find_host_call as usize);
        rust_main()
    }
}
