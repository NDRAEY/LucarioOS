#![no_std]
#![no_main]
#![no_builtins]
#![feature(lang_items)]
#![feature(panic_info_message)]

#[macro_use]
mod log;
mod conv;
mod display;
mod multiboot;
mod ports;

use core::panic::PanicInfo;

use multiboot::MultibootHeader;

use crate::{display::real_canvas::Canvas, log::log::DEBUG_PORT, ports::com_init};


#[no_mangle]
#[allow(arithmetic_overflow)]
pub unsafe extern "C" fn _start(multiboot_addr: u32, stack_top: u32) -> ! {
    com_init(DEBUG_PORT);

    debug!("Hello world from Rust!");

    let mb: *const MultibootHeader = multiboot_addr as *const MultibootHeader;
    let addr = (*mb).framebuffer_addr as usize;

    let width = unsafe { (*mb).framebuffer_width } as usize;
    let height = unsafe { (*mb).framebuffer_height } as usize;
    let fb_bpp = unsafe { (*mb).framebuffer_bpp } as usize;
    let fb_pitch = unsafe { (*mb).framebuffer_pitch } as usize;
    let height = unsafe { (*mb).framebuffer_height } as usize;

    log::log::debug_write_string("Screen address: ");
    log::log::debug_write_hexadecimal_unsigned(addr as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Screen width: ");
    log::log::debug_write_number(width as _);
    log::log::debug_write_char(b'\n');

    log::log::debug_write_string("Screen height: ");
    log::log::debug_write_number(height as _);
    log::log::debug_write_char(b'\n');

    let canvas = Canvas {
        buffer: addr as *mut u8,
        width,
        height,
        pitch: fb_pitch,
        bpp: fb_bpp,
    };

    let mut i: usize = 0;

	loop {		
	    for y in 0..height {
	        for x in 0..width {
	            canvas.pixel(
	                x,
	                y,
	                draw_fn(x, y, i) as u32
	            );
	        }
	    }

	    i += 1;
	}

    loop {}
}

fn draw_fn(x: usize, y: usize, i: usize) -> usize {
	x.wrapping_div(y)
}

#[no_mangle]
pub unsafe extern "C" fn memset(pointer: *mut u8, value: u8, count: usize) {
    let mut c = count;

    while c > 0 {
        *pointer.offset(c as isize).as_mut().unwrap_unchecked() = value;
        c -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(destination: *mut u8, source: *const u8, count: usize) {
    let mut c = count;

    while c > 0 {
        *destination.offset(c as isize).as_mut().unwrap_unchecked() = *source.offset(c as isize);
        c -= 1;
    }
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
