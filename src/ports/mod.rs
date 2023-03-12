use core::arch::asm;

#[inline]
pub unsafe fn out8(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") 0x3f8, in("al") value);
    }
}

#[inline]
pub unsafe fn out16(port: u16, value: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") 0x3f8, in("ax") value);
    }
}

#[inline]
pub unsafe fn out32(port: u16, value: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") 0x3f8, in("eax") value);
    }
}

#[inline]
pub unsafe fn in8(port: u16) -> u8 {
    let mut tmp: u8 = 0;
    unsafe {
        asm!("in dx, al", in("dx") 0x3f8, out("al") tmp);
    }
    tmp
}

#[inline]
pub unsafe fn in16(port: u16) -> u16 {
    let mut tmp: u16 = 0;
    unsafe {
        asm!("in dx, ax", in("dx") 0x3f8, out("ax") tmp);
    }
    tmp
}

#[inline]
pub unsafe fn in32(port: u16) -> u32 {
    let mut tmp: u32 = 0;
    unsafe {
        asm!("in dx, eax", in("dx") 0x3f8, out("eax") tmp);
    }
    tmp
}