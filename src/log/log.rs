pub use crate::ports::out8;
pub use crate::conv::itoa::itoa_bytes;

const DEBUG_PORT: u16 = 0x3f8;

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
        crate::debug_str_nonl!("[LOG ", file!(), ":", "--", "] ");
        $(
            crate::debug_str_nonl!($message);
        )*
        crate::log::log::debug_write_string("\n");
    };
}

#[inline]
pub fn debug_write_char(chr: u8) {
    unsafe {
        out8(DEBUG_PORT, chr);
    }
}

#[inline]
pub fn debug_write_string(strng: &str) {
    for i in strng.as_bytes() {
        debug_write_char(*i);
    }
}

#[inline]
pub fn debug_write_number(num: isize) {
	let mut buf: [u8; 12] = [0; 12];
    // let mut n = num;
	// let mut idx = 0;

    let length = itoa_bytes(num , &mut buf);

    for i in 0..length {
        debug_write_char(buf[i]);
    }
}