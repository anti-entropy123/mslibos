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
use ms_std::{args, println, libos::libos};
use ms_hostcall::types::{OpenFlags, OpenMode};

use tinywasm::{Module, Store, ModuleInstance};
use wasi_api::tinywasm;

const WASM: &[u8] = include_bytes!("../sorter.wasm");

lazy_static::lazy_static! {
    static ref MUST_OPEN_ROOT: bool = {
        libos!(open("/", OpenFlags::empty(), OpenMode::RD)).unwrap();
        true
    };
}

fn func_body(my_id: &str, sorter_num: u64, merger_num: u64) -> Result<()> {
    #[cfg(feature = "log")]
    println!("rust: my_id: {:?}, reducer_num: {:?}", my_id, reducer_num);

    let mut wasi_args: Vec<String> = Vec::new();
    wasi_args.push("fake system path!".to_string()); // c语言main第一个参数是系统路径
    wasi_args.push(my_id.to_string());
    wasi_args.push(sorter_num.to_string());
    wasi_args.push(merger_num.to_string());

    let _open_root = *MUST_OPEN_ROOT;

    // let data_fd = libos!(open(&format!("little_fake_data_{}.txt", my_id), OpenFlags::O_CREAT, OpenMode::RDWR))? as u32;
    // match my_id {
    //     "0" => {
    //         libos!(write(data_fd, b"34 12 7 26 1 19 30 3 38 9\n"))?;
    //     },
    //     "1" => {
    //         libos!(write(data_fd, b"0 24 22 14 36 17 15 32 6 4\n"))?;
    //     },
    //     "2" => {
    //         libos!(write(data_fd, b"20 37 8 35 31 29 25 5 10 11\n"))?;
    //     },
    //     "3" => {
    //         libos!(write(data_fd, b"18 33 2 16 39 13 28 27 21 23\n"))?;
    //     },
    //     _ => {
    //         libos!(write(data_fd, b"7 4 8 5 1 3 9 6 2\n"))?;
    //     }
    // }
    // libos!(close(data_fd))?;

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

#[no_mangle]
pub fn main() -> Result<()> {
    let my_id = args::get("id").unwrap();
    let sorter_num: u64 = args::get("sorter_num")
        .expect("missing arg sorter_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, sorter_num={}", args::get("sorter_num").unwrap()));
    let merger_num: u64 = args::get("merger_num")
        .expect("missing arg merger_num")
        .parse()
        .unwrap_or_else(|_| panic!("bad arg, merger_num={}", args::get("merger_num").unwrap()));

    func_body(my_id, sorter_num, merger_num)
}