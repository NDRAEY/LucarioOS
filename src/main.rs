#![no_std]
#![no_main]
#![no_builtins]
#![feature(lang_items)]
#![feature(panic_info_message)]

mod multiboot;
mod ports;

use core::panic::PanicInfo;

#[macro_use]
mod log;

mod conv;

use multiboot::MultibootHeader;

use crate::conv::itoa::itoa_bytes_universal;
use crate::{log::log::DEBUG_PORT, ports::com_init};

#[no_mangle]
pub extern "C" fn _start(multiboot2_stack: u32, multiboot_structure_addr: u32) -> ! {
    com_init(DEBUG_PORT);

    debug!("Hello world from Rust!");

    // fault!("Hello world!");

    let mb: *mut MultibootHeader = multiboot_structure_addr as *mut MultibootHeader;
    let addr = unsafe { (*mb).framebuffer_addr };

    let test = 123456;

    log::log::debug_write_string("Test: ");
    log::log::debug_write_hexadecimal(test as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Test: ");
    log::log::debug_write_hexadecimal(-test as _);
    log::log::debug_write_char(b'\n');
    
    log::log::debug_write_string("Test: ");
    log::log::debug_write_number(test as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Test: ");
    log::log::debug_write_number(-test as _);
    log::log::debug_write_char(b'\n');

    let width = unsafe { (*mb).framebuffer_width } as usize;
    let height = unsafe { (*mb).framebuffer_height } as usize;

    log::log::debug_write_string("MB structure address: ");
    log::log::debug_write_hexadecimal(multiboot_structure_addr as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Stack address: ");
    log::log::debug_write_hexadecimal(multiboot2_stack as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Screen address: ");
    log::log::debug_write_hexadecimal(addr as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Screen width: ");
    log::log::debug_write_number(width as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Screen height: ");
    log::log::debug_write_number(height as _);
    log::log::debug_write_char(b'\n');

    // debug!("Positive: ", test, "\nNegative: ", -test);

    // panic!();

    // let buffer: &mut [u8] = unsafe { slice::from_raw_parts_mut(addr as *mut u8, width * height * 3) };

    // for y in 0..height {
    // 	for x in 0..width {
    // 		buffer[x * 3 + (y * width * 3)] = 255;
    // 		buffer[x * 3 + (y * width * 3) + 1] = 255;
    // 		buffer[x * 3 + (y * width * 3) + 2] = 255;
    // 	}
    // }

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn __eh_personality() {}

#[panic_handler]
#[no_mangle]
extern "C" fn __panic_handler(info: &PanicInfo) -> ! {
    debug!("Panic encountered! ", file!(), " : --");
    // debug!("Panic! Message: ", info.message().unwrap().as_str().unwrap());
    loop {}
}
