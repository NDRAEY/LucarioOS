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

use crate::{display::{real_canvas::Canvas, console::TTY}, log::log::*, ports::com_init};
use crate::conv::fmt::Hexadecimal;

#[no_mangle]
#[allow(arithmetic_overflow)]
pub unsafe extern "C" fn _start(multiboot_addr: u32, _stack_top: u32) -> ! {
    com_init(DEBUG_PORT);

    debug!("Hello world from Rust!", 12345);

    let mb: *const MultibootHeader = multiboot_addr as *const MultibootHeader;
    let addr = (*mb).framebuffer_addr as usize;

    let width = unsafe { (*mb).framebuffer_width } as usize;
    let height = unsafe { (*mb).framebuffer_height } as usize;
    let fb_bpp = unsafe { (*mb).framebuffer_bpp } as usize;
    let fb_pitch = unsafe { (*mb).framebuffer_pitch } as usize;
    let height = unsafe { (*mb).framebuffer_height } as usize;

    debug!("Screen address:", Hexadecimal::Unsigned(addr));
    debug!("Screen width:", width);
    debug!("Screen height:", height);
    
    let canvas = Canvas {
        buffer: addr as *mut u8,
        width,
        height,
        pitch: fb_pitch,
        bpp: fb_bpp,
    };

    let mut console = TTY {
        canvas,
        x: 0,
        y: 0,
        color: 0xffffff
    };

    console.puts("Hyvaa yota, Valery Artemovich!\n");
    console.puts("0_0 I made a console?\n");

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn __eh_personality() {}

#[panic_handler]
#[no_mangle]
extern "C" fn __panic_handler(info: &PanicInfo) -> ! {
    // debug!("Panic encountered! ", file!(), " : --");
    // debug!("Panic! Message: ", info.message().unwrap().as_str().unwrap());
    loop {}
}
