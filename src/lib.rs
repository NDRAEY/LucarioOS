#![no_std]
#![no_main]
#![no_builtins]
#![feature(lang_items)]
#![feature(panic_info_message)]

#[macro_use]
mod log;
mod conv;
mod display;
mod gdt;
mod interrupts;
mod mem;
mod multiboot;
mod ports;
mod stubs;

use core::panic::PanicInfo;

use multiboot::MultibootHeader;

use crate::conv::fmt::Hexadecimal;
use crate::{
    display::{console::TTY, real_canvas::Canvas},
    log::log::*,
    multiboot::MultibootModListEntry,
    ports::com_init,
};

#[no_mangle]
//:#[allow(arithmetic_overflow)]
pub unsafe extern "C" fn _start(multiboot_addr: u32, _stack_top: u32) -> ! {
    com_init(DEBUG_PORT);

    debug!(env!("CARGO_PKG_VERSION"));
    gdt::init();
    interrupts::init();

    debug!("Hello world from Rust!", 12345);
    debug!("Size:", core::mem::size_of::<MultibootHeader>());
    let mb: *const MultibootHeader = multiboot_addr as *const MultibootHeader;

    {
        debug!("MBaddr:", Hexadecimal::Unsigned(multiboot_addr as usize));
    }

    let ml = (*mb).mmap_length;
    let ss = (*mb).mmap_addr;
    debug!("Memory map:", Hexadecimal::Unsigned(ss as usize), ml);
    
    let mut addr = (*mb).framebuffer_addr as usize;

    let width = (*mb).framebuffer_width as usize;
    let height = (*mb).framebuffer_height as usize;
    //let fb_bpp = mb.framebuffer_bpp as usize;
    //let fb_pitch = mb.framebuffer_pitch as usize;

    debug!("Creating memory manager");

    mem::physical::ginit(mb);

    debug!("Initializing...");

    /*debug!("Found", memory_manager.available());

    debug!("Screen address:", Hexadecimal::Unsigned(addr));
    debug!("Screen width:", width);
    debug!("Screen height:", height);

    let canvas = Canvas::from_multiboot(mb);  // Needs memory?

    let mut console = TTY::new(&canvas);

    console.puts("Hyvaa yota, Valery Artemovich!\n");
    console.puts("0_0 I made a console?\n");

    canvas.pixel(40, 50, 0xff0000);

    // panic!("WHAT?");
    */
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn __eh_personality() {}

#[panic_handler]
#[no_mangle]
fn __panic_handler(info: &PanicInfo) -> ! {
    debug!(
        "Panic encountered! ",
        info.location().unwrap().file(),
        info.location().unwrap().line()
    );
    debug!(
        "Panic! Message: ",
        info.message().unwrap().as_str().unwrap()
    );
    loop {}
}
