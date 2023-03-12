#![no_std]
#![no_main]

#![feature(lang_items)]
#![feature(panic_info_message)]

mod multiboot;
mod ports;

#[macro_use]
mod log;

use multiboot::MultibootHeader;

#[no_mangle]
pub extern "C" fn _start(multiboot2_stack: u32, multiboot_structure_addr: u32) -> ! {
	debug!("Hello world from Rust!");

	let mb: *mut MultibootHeader = multiboot_structure_addr as *mut MultibootHeader;
	let addr = unsafe { (*mb).framebuffer_addr };


	let width = unsafe { (*mb).framebuffer_width } as usize;
	let height = unsafe { (*mb).framebuffer_height } as usize;

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
extern "C" fn eh_personality() {}

#[panic_handler]
fn __panic_hdl(_info: &core::panic::PanicInfo) -> ! {
	loop {}	
}
