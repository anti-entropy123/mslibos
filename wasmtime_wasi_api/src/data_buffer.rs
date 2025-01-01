extern crate alloc;
use core::mem::forget;

use alloc::{string::String, vec::Vec};

use ms_std::agent::DataBuffer;
#[cfg(feature = "log")]
use ms_std::{println, time::{SystemTime, UNIX_EPOCH}};
use ms_std_proc_macro::FaasData;
use wasmtime::Caller;

use crate::LibosCtx;

#[derive(FaasData)]
struct WasmDataBuffer(*mut u8, usize);

impl Default for WasmDataBuffer {
    fn default() -> Self {
        Self(core::ptr::null_mut(), Default::default())
    }
}

pub fn buffer_register(
    mut caller: Caller<'_, LibosCtx>,
    slot_name_base: i32, slot_name_size: i32, buffer_offset: i32, buffer_size: i32,
) {
    #[cfg(feature = "log")]
    {
        println!("[Debug] buffer_register");
        println!("[Time] buffer_register: {}", SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64);
    }

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut slot_name: Vec<u8> = Vec::with_capacity(slot_name_size as usize);
    slot_name.resize(slot_name_size as usize, 0);
    memory.read(&caller, slot_name_base as usize, &mut slot_name).unwrap();
    let slot_name = String::from_utf8(slot_name).expect("[Err] Not a valid UTF-8 sequence");

    #[cfg(feature = "log")]
    println!("slot_name={}", slot_name);

    let content = memory.data_mut(&mut caller)
                                    .get_mut(buffer_offset as usize..)
                                    .and_then(|s| s.get_mut(..buffer_size as usize))
                                    .unwrap();

    let buffer_base = content.as_mut_ptr();

    #[cfg(feature = "log")]
    println!("base={:?}, addr={:?}, offset={:?}, size={}", base, buffer_base, buffer_offset, buffer_size);
    #[cfg(feature = "log")]
    println!("content={:?}", content);

    let mut wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::with_slot(slot_name);
    wasm_buffer.0 = buffer_base;
    wasm_buffer.1 = buffer_size as usize;
}

pub fn access_buffer(
    mut caller: Caller<'_, LibosCtx>,
    slot_name_base: i32, slot_name_size: i32, buffer_base: i32, buffer_size: i32,
) {
    #[cfg(feature = "log")]
    {
        println!("[Debug] access_buffer");
        println!("[Time] access_buffer: {}", SystemTime::now().duration_since(UNIX_EPOCH).as_micros() as f64 / 1000000f64);
    }

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut slot_name: Vec<u8> = Vec::with_capacity(slot_name_size as usize);
    slot_name.resize(slot_name_size as usize, 0);
    memory.read(&caller, slot_name_base as usize, &mut slot_name).unwrap();
    let slot_name = String::from_utf8(slot_name).expect("[Err] Not a valid UTF-8 sequence");

    #[cfg(feature = "log")]
    println!("slot_name={}", slot_name);
    let wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::from_buffer_slot(slot_name).unwrap();
    #[cfg(feature = "log")]
    println!(
        "wasm_buffer -> addr={:?}, size={}",
        wasm_buffer.0, wasm_buffer.1
    );

    if buffer_size as usize != wasm_buffer.1 {
        panic!("buffer_size={}, wasm_buffer.1={}, access_buffer's size is different from buffer_register's size", buffer_size, wasm_buffer.1)
    }
    let buffer = unsafe { core::slice::from_raw_parts(wasm_buffer.0, wasm_buffer.1) };
    #[cfg(feature = "log")]
    println!("buffer: {:?}", buffer);
    memory.write(&mut caller, buffer_base as usize, buffer).unwrap();
}
