pub use crate::ports::out8;
use crate::{
    conv::itoa::{itoa_bytes_universal, itoa_bytes_universal_unsigned},
    ports::in8,
};

pub const DEBUG_PORT: u16 = 0x3f8;

#[macro_export]
macro_rules! debug_str_nonl {
    ( $( $x:expr ),* ) => {
        $(
            crate::log::log::debug_write_string($x);
        )*
    }
}

#[macro_export]
macro_rules! debug_str {
    ( $( $x:expr ),* ) => {
        $(
            crate::log::log::debug_write_string($x);
        )*
        crate::log::log::debug_write_string("\n");
    }
}

#[macro_export]
macro_rules! debug {
    ( $( $message:expr ),* ) => {
        {
        crate::debug_str_nonl!("[LOG ", file!(), ":");
        crate::log::log::debug_write_number(line!() as _);
        crate::debug_str_nonl!("] ");
        $(
            crate::debug_str_nonl!($message);
        )*
        crate::log::log::debug_write_string("\n");
        }
    };
}

#[inline]
pub fn debug_write_char(chr: u8) {
    unsafe {
        while com_port_busy(DEBUG_PORT) {}
        out8(DEBUG_PORT, chr);
    }
}

#[inline]
pub fn com_port_busy(port: u16) -> bool {
    unsafe { (in8(port + 5) & 0x20) == 1 }
}

#[inline]
pub fn debug_write_string(strng: &str) {
    for i in strng.as_bytes() {
        debug_write_char(*i);
    }
}

#[inline]
pub fn debug_write_number(num: isize) {
    let mut buf: [u8; 33] = [0; 33];
    let length = itoa_bytes_universal(num, &mut buf, 10);
    let mut i = 0;

    while i < length {
        unsafe {
            debug_write_char(*buf.get(i).unwrap_unchecked());
        }

        i += 1;
    }
}

#[inline]
pub fn debug_write_hexadecimal(num: isize) {
    let mut buf: [u8; 33] = [0; 33];
    let length = itoa_bytes_universal(
        num.abs(),
        &mut buf,
        16
    );

    let mut i = 0;

    if num < 0 {
        debug_write_char(b'-');
    }

    debug_write_string("0x");

    while i < length {
        unsafe {
            debug_write_char(*buf.get(i).unwrap_unchecked());
        }

        i += 1;
    }
}

#[inline]
pub fn debug_write_hexadecimal_unsigned(num: usize) {
    let mut buf: [u8; 33] = [0; 33];
    let length = itoa_bytes_universal_unsigned(num, &mut buf, 16);
    let mut i = 0;

    if num < 0 {
        debug_write_char(b'-');
    }

    debug_write_string("0x");

    while i < length {
        unsafe {
            debug_write_char(*buf.get(i).unwrap_unchecked());
        }

        i += 1;
    }
}

#[inline]
pub fn debug_write_binary(num: usize) {
    let mut buf: [u8; 33] = [0; 33];
    let length = itoa_bytes_universal_unsigned(num, &mut buf, 2);
    let mut i = 0;

    if num < 0 {
        debug_write_char(b'-');
    }

    debug_write_string("0b");

    while i < length {
        unsafe {
            debug_write_char(*buf.get(i).unwrap_unchecked());
        }

        i += 1;
    }
}