extern crate alloc;
use alloc::{string::String, vec::Vec};

use core::mem::forget;

use ms_std::agent::DataBuffer;
#[cfg(feature = "log")]
use ms_std::println;
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
    slot_name_base: i32, slot_name_size: i32, buffer_base: i32, buffer_size: i32,
) {
    #[cfg(feature = "log")]
    println!("[Debug] buffer_register");

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut slot_name: Vec<u8> = Vec::with_capacity(slot_name_size as usize);
    slot_name.resize(slot_name_size as usize, 0);
    memory.read(&caller, slot_name_base as usize, &mut slot_name).unwrap();
    let slot_name = String::from_utf8(slot_name).expect("[Err] Not a valid UTF-8 sequence");

    #[cfg(feature = "log")]
    println!("slot_name={}", slot_name);

    let mut content: Vec<u8> = Vec::with_capacity(buffer_size as usize);
    content.resize(buffer_size as usize, 0);
    memory.read(&caller, buffer_base as usize, &mut content).unwrap();
    let mut content = String::from_utf8(content).expect("[Err] Not a valid UTF-8 sequence");

    let buffer_base = content.as_mut_ptr();
    #[cfg(feature = "log")]
    println!("content={}", content);
    forget(content);

    let mut wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::with_slot(slot_name);
    wasm_buffer.0 = buffer_base;
    wasm_buffer.1 = buffer_size as usize;
}

pub fn access_buffer(
    mut caller: Caller<'_, LibosCtx>,
    slot_name_base: i32, slot_name_size: i32, buffer_base: i32, buffer_size: i32,
) {
    #[cfg(feature = "log")]
    println!("[Debug] access_buffer");

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
        panic!()
    }
    let buffer = unsafe { core::slice::from_raw_parts(wasm_buffer.0, wasm_buffer.1) };
    #[cfg(feature = "log")]
    println!("buffer: {:?}", buffer);
    memory.write(&mut caller, buffer_base as usize, buffer).unwrap();
}
