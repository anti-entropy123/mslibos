use alloc::format;

use crate::libos;
use core::fmt;

#[allow(unused)]
pub fn print(args: fmt::Arguments) {
    libos::host_write(1, &format!("{}", args));
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
