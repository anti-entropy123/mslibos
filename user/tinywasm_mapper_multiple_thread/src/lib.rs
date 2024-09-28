#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        extern crate alloc;
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use alloc::{collections::BTreeMap, format, string::{String, ToString}, vec::Vec};
use ms_std::println;
use ms_std::libos::libos;
use ms_hostcall::types::{OpenFlags, OpenMode};

use tinywasm::{Module, Store, ModuleInstance};
use wasi_api::tinywasm;

const WASM: &[u8] = include_bytes!("../mapper.wasm");

lazy_static::lazy_static! {
    static ref MUST_OPEN_ROOT: bool = {
        libos!(open("/", OpenFlags::empty(), OpenMode::RD)).unwrap();
        true
    };
}

#[no_mangle]
pub fn main(args: &BTreeMap<String, String>) -> Result<()> {
    let my_id = &args["id"];
    let reducer_num: u64 = args
        .get("reducer_num")
        .expect("missing arg reducer_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, reducer_num={}", args["reducer_num"]));
    println!("rust: my_id: {:?}, reducer_num: {:?}", my_id, reducer_num);
    let mut wasi_args: Vec<String> = Vec::new();
    wasi_args.push("fake system path!".to_string()); // c语言main第一个参数是系统路径
    wasi_args.push(my_id.to_string());
    wasi_args.push(reducer_num.to_string());

    let _open_root = *MUST_OPEN_ROOT;

    let data_fd = libos!(open(&format!("little_fake_data_{}.txt", my_id), OpenFlags::O_CREAT, OpenMode::RDWR))? as u32;
    println!("mapper_{} data_fd: {}", my_id, data_fd);
    libos!(write(data_fd, b"hello hello hello hello name name name name world world world world\n"))?;
    libos!(write(data_fd, b"you you you you like like like like potato potato potato potato\n"))?;
    libos!(close(data_fd))?;

    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    wasi_api::set_wasi_args(store.id(), wasi_args);

    let instance = ModuleInstance::instantiate(&mut store, module, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;

    if let Err(e) = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap()) {
        let msg = format!("{:?}", e);
        println!("err msg: {}", msg);
        if msg != "normally exit" {
            // return Err();
        }
    };

    Ok(().into())
}
