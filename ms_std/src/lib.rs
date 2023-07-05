#![no_std]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(ip_in_core)]
#![feature(decl_macro)]
#![feature(concat_idents)]
#![feature(generic_const_exprs)]
#![allow(clippy::result_unit_err)]
#![allow(incomplete_features)]

use core::{mem::size_of, ptr, slice};

use alloc::vec::Vec;

pub mod console;

pub mod init_context;
pub mod libos;
pub mod net;
pub mod sync;

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc_def")] {
        mod heap_alloc;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "unwinding", feature = "panic_def"))] {
        compile_error!("must only choose one in 'unwinding' and 'panic_def'");
    } else
    if #[cfg(feature = "panic_def")] {
        mod panic_def {
            use crate::init_context::isolation_ctx;
            use core::panic::PanicInfo;

            #[panic_handler]
            fn panic_handler(_info: &PanicInfo) -> ! {
                let panic_addr = isolation_ctx().panic_handler;

                let host_panic_handler: unsafe extern "C" fn() -> ! =
                    unsafe { core::mem::transmute(panic_addr) };
                unsafe { host_panic_handler() }
            }

            #[lang = "eh_personality"]
            extern "C" fn eh_personality() {}
        }
    }
}

pub struct DataBuffer {
    inner: Vec<u8>,
}

impl DataBuffer {
    pub fn new<T>(mut raw: T) -> DataBuffer {
        let p = ptr::addr_of_mut!(raw) as usize;
        let p = p as *mut u8;
        DataBuffer {
            inner: Vec::from(unsafe { slice::from_raw_parts::<u8>(p, size_of::<T>()) }),
        }
    }

    pub fn to<'a, T>(&'a mut self) -> &'a mut T {
        assert_eq!(size_of::<T>(), self.inner.len());
        let p = self.inner.as_mut_ptr() as usize as *mut T;
        unsafe { &mut *p }
    }
}

#[linkage = "weak"]
#[no_mangle]
pub fn main() -> Result<DataBuffer, ()> {
    panic!("need real main");
}

#[no_mangle]
pub fn rust_main() -> Result<(), ()> {
    #[cfg(feature = "unwinding")]
    {
        use unwinding::panic;
        let result = panic::catch_unwind(main);

        if let Err(ref e) = result {
            println!("error: {:#?}", e)
        }

        result.map_err(|_| ())
    }
    #[cfg(not(feature = "unwinding"))]
    {
        let r = main();
        assert!(r.is_ok());
        Ok(())
    }
}
