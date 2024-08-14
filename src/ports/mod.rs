use core::arch::asm;

#[inline(always)]
pub unsafe fn out8(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

#[inline]
pub unsafe fn out16(port: u16, value: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
}

#[inline]
pub unsafe fn out32(port: u16, value: u32) {
    asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn in8(port: u16) -> u8 {
    let mut tmp: u8 = 0;
    asm!("in al, dx", out("al") tmp, in("dx") port, options(nomem, nostack, preserves_flags));
    tmp
}

#[inline]
pub unsafe fn in16(port: u16) -> u16 {
    let mut tmp: u16 = 0;
    unsafe {
        asm!("in ax, dx", out("ax") tmp, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    tmp
}

#[inline]
pub unsafe fn in32(port: u16) -> u32 {
    let mut tmp: u32 = 0;
    unsafe {
        asm!("in dx, eax", in("dx") port, out("eax") tmp, options(nomem, nostack, preserves_flags));
    }
    tmp
}

pub fn com_init(port: u16) -> bool {
    unsafe {
        out8(port + 1, 0x00); // Disable all interrupts
        out8(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
        out8(port + 0, 0x01); // Set divisor to 1 (lo byte) 115200 / divisor (1) = 115200 baud
        out8(port + 1, 0x00); //                  (hi byte)
        out8(port + 3, 0x03); // 8 bits, no parity, one stop bit
        out8(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        out8(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        out8(port + 4, 0x1E); // Set in loopback mode, test the serial chip
        out8(port + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        if in8(port + 0) != 0xAE {
            return false;
        }

        out8(port + 4, 0x0F);
    }
    return true;
}
