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

use multiboot::MultibootHeader;

fn print_number(num: isize) {
	let mut buf: [u8; 11] = [0; 11];
    let mut n = num;
	let mut idx = 0;

    if n < 0 {
        log::log::debug_write_char(b'-');
        n = -n;
    }

    while n >= 10 {
    	buf[idx] = b'0' + (n % 10) as u8;
    	idx += 1;
		n /= 10;
    }

   	buf[idx] = b'0' + (n % 10) as u8;
	idx += 1;

    while idx > 0 {
    	log::log::debug_write_char(buf[idx]);
    	idx -= 1;
    }
    
   	log::log::debug_write_char(buf[idx]);
}

#[no_mangle]
pub extern "C" fn _start(multiboot2_stack: u32, multiboot_structure_addr: u32) -> ! {
    debug!("Hello world from Rust!");

    // fault!("Hello world!");

    let mb: *mut MultibootHeader = multiboot_structure_addr as *mut MultibootHeader;
    let addr = unsafe { (*mb).framebuffer_addr };

    print_number(-12345678);

    // let width = unsafe { (*mb).framebuffer_width } as usize;
    // let height = unsafe { (*mb).framebuffer_height } as usize;

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
