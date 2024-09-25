use ms_std::{agent::DataBuffer, println};
use ms_std_proc_macro::FaasData;
use tinywasm::{FuncContext, MemoryStringExt};

#[derive(FaasData)]
struct WasmDataBuffer(*mut u8, usize);

impl Default for WasmDataBuffer {
    fn default() -> Self {
        Self(core::ptr::null_mut(), Default::default())
    }
}

pub fn buffer_register(
    mut ctx: FuncContext,
    (slot_name_base, slot_name_size, buffer_base, buffer_size): (i32, i32, i32, i32),
) -> tinywasm::Result<()> {
    println!("buffer_register");

    let memory = ctx.exported_memory("memory").unwrap();
    let slot_name = memory
        .load_string(slot_name_base as usize, slot_name_size as usize)
        .unwrap();
    let mut wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::with_slot(slot_name);
    wasm_buffer.0 = buffer_base as usize as *mut _;
    wasm_buffer.1 = buffer_size as usize;

    Ok(())
}

pub fn access_buffer(
    mut ctx: FuncContext,
    (slot_name_base, slot_name_size, buffer_base, buffer_size): (i32, i32, i32, i32),
) -> tinywasm::Result<()> {
    let mut memory = ctx.exported_memory_mut("memory").unwrap();
    let slot_name = memory
        .load_string(slot_name_base as usize, slot_name_size as usize)
        .unwrap();

    let wasm_buffer: DataBuffer<WasmDataBuffer> = DataBuffer::from_buffer_slot(slot_name).unwrap();
    if buffer_size as usize != wasm_buffer.1 {
        panic!()
    }
    memory
        .store(buffer_base as usize, buffer_size as usize, unsafe {
            core::slice::from_raw_parts(wasm_buffer.0, wasm_buffer.1)
        })
        .unwrap();

    Ok(())
}
