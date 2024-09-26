#![cfg_attr(feature = "with_libos", no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "with_libos")] {
        use ms_std::{agent::FaaSFuncResult as Result};
        extern crate alloc;
        use alloc::{collections::BTreeMap, string::String};
    } else {
        type Result<T> = core::result::Result<T, String>;
        use std::collections::BTreeMap;
    }
}

use alloc::{format, vec};
use alloc::vec::Vec;
use ms_std::{
    libos::{libos},
    println,
    time::{SystemTime, UNIX_EPOCH},
};
use wasi_api::tinywasm;
use tinywasm::Extern;
use tinywasm::{FuncContext, Imports, Module, Store};

// use ms_std::libos::libos;

#[repr(C)]
struct WasiCiovec {
    buf: u32,
    buf_len: u32,
}

struct LCG {
    state: u64,
}

impl LCG {
    fn new(seed: u64) -> Self {
        LCG { state: seed }
    }

    fn next_u8(&mut self) -> u8 {
        // LCG的参数
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        const MOD: u64 = 1 << 32;

        // 更新状态
        self.state = (A.wrapping_mul(self.state).wrapping_add(C)) % MOD;

        // 返回一个0到255之间的随机u8
        (self.state % 256) as u8
    }

    fn generate_random_u8_slice(&mut self, length: usize) -> Vec<u8> {
        (0..length).map(|_| self.next_u8()).collect()
    }
}

const WASM: &[u8] = include_bytes!("../rustpython_hello.wasm");

#[no_mangle]
pub fn main(_args: &BTreeMap<String, String>) -> Result<()> {
    let module = Module::parse_bytes(WASM)?;
    let mut store = Store::default();
    let imports = wasi_api::import_all()?;

    let instance = module.instantiate(&mut store, Some(imports))?;
    let main = instance.exported_func::<(), ()>(&store, "_start")?;
    
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();

    if let Err(e) = unwinding::panic::catch_unwind(|| main.call(&mut store, ()).unwrap()) {
        let msg = format!("{:?}", e);
        println!("err msg: {:?}", msg);
        if msg != "normally exit" {
            // return Err();
        }
    };

    Ok(().into())
}
