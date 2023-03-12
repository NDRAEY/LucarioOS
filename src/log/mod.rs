use crate::ports::out8;

const DEBUG_PORT: u16 = 0x3f8;

macro_rules! debug_str {
    ( $( $x:expr ),* ) => {
        $(
            crate::log::debug_write_string($x);
        )*
        crate::log::debug_write_string("\n");
    }
}

macro_rules! debug {
    ( $( $message:expr ),* ) => {
        $(
            debug_str!("[LOG ", file!(), ":", "--", "] ", $message);
        )*
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
