use crate::conv::fmt::Hexadecimal;
use crate::ports::out8;

extern "C" {
    fn idt_flush(pointer: *const IDTPointer);

    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();

    fn irq0();
    fn irq1();
    fn irq2();
    fn irq3();
    fn irq4();
    fn irq5();
    fn irq6();
    fn irq7();
    fn irq8();
    fn irq9();
    fn irq10();
    fn irq11();
    fn irq12();
    fn irq13();
    fn irq14();
    fn irq15();

    fn isr80();
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IDTEntry {
    base_low: u16,
    selector: u16,
    always_zero: u8,
    flags: u8,
    base_high: u16,
}

#[repr(C)]
#[repr(packed)]
struct IDTPointer {
    limit: u16,
    base: *const IDTEntry,
}

#[repr(C)]
pub struct X86Registers {
    ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    int_num: u32,
    err_code: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32,
}

static mut IDT_ENTRIES: [IDTEntry; 256] = [IDTEntry {
    base_low: 0,
    selector: 0,
    always_zero: 0,
    flags: 0,
    base_high: 0,
}; 256];

static mut IDT_POINTER: IDTPointer = IDTPointer {
    limit: 0,
    base: core::ptr::null(),
};

use crate::log::log::DebugWrite;

use crate::debug;

pub unsafe fn init() {
    core::arch::asm!("cli");

    //debug!("Start", Hexadecimal::Unsigned(&IDT_ENTRIES as *const _ as usize));
    //debug!("Start", Hexadecimal::Unsigned(&IDT_POINTER as *const _ as usize));

    IDT_POINTER.base = &IDT_ENTRIES as *const IDTEntry;
    //IDT_POINTER.base = core::mem::transmute(&IDT_ENTRIES);

    IDT_POINTER.limit = ((core::mem::size_of::<IDTEntry>() * 256) - 1) as u16;

    out8(0x20, 0x11);
    out8(0xA0, 0x11);

    out8(0x21, 0x20);
    out8(0xA1, 0x28);

    out8(0x21, 0x04);
    out8(0xA1, 0x02);

    out8(0x21, 0x01);
    out8(0xA1, 0x01);

    out8(0x21, 0x0);
    out8(0xA1, 0x0);

    core::arch::asm!("nop");

    set_gate(0, isr0 as u32, 0x08, 0x8E);
    set_gate(1, isr1 as u32, 0x08, 0x8E);
    set_gate(2, isr2 as u32, 0x08, 0x8E);
    set_gate(3, isr3 as u32, 0x08, 0x8E);
    set_gate(4, isr4 as u32, 0x08, 0x8E);
    set_gate(5, isr5 as u32, 0x08, 0x8E);
    set_gate(6, isr6 as u32, 0x08, 0x8E);
    set_gate(7, isr7 as u32, 0x08, 0x8E);

    set_gate(8, isr8 as u32, 0x08, 0x8E);
    set_gate(9, isr9 as u32, 0x08, 0x8E);
    set_gate(10, isr10 as u32, 0x08, 0x8E);
    set_gate(11, isr11 as u32, 0x08, 0x8E);
    set_gate(12, isr12 as u32, 0x08, 0x8E);
    set_gate(13, isr13 as u32, 0x08, 0x8E);
    set_gate(14, isr14 as u32, 0x08, 0x8E);
    set_gate(15, isr15 as u32, 0x08, 0x8E);

    set_gate(16, isr16 as u32, 0x08, 0x8E);
    set_gate(17, isr17 as u32, 0x08, 0x8E);
    set_gate(18, isr18 as u32, 0x08, 0x8E);
    set_gate(19, isr19 as u32, 0x08, 0x8E);
    set_gate(20, isr20 as u32, 0x08, 0x8E);
    set_gate(21, isr21 as u32, 0x08, 0x8E);
    set_gate(22, isr22 as u32, 0x08, 0x8E);
    set_gate(23, isr23 as u32, 0x08, 0x8E);

    set_gate(24, isr24 as u32, 0x08, 0x8E);
    set_gate(25, isr25 as u32, 0x08, 0x8E);
    set_gate(26, isr26 as u32, 0x08, 0x8E);
    set_gate(27, isr27 as u32, 0x08, 0x8E);
    set_gate(28, isr28 as u32, 0x08, 0x8E);
    set_gate(29, isr29 as u32, 0x08, 0x8E);
    set_gate(30, isr30 as u32, 0x08, 0x8E);
    set_gate(31, isr31 as u32, 0x08, 0x8E);

    set_gate(32, irq0 as u32, 0x08, 0x8E);
    set_gate(33, irq1 as u32, 0x08, 0x8E);
    set_gate(34, irq2 as u32, 0x08, 0x8E);
    set_gate(35, irq3 as u32, 0x08, 0x8E);
    set_gate(36, irq4 as u32, 0x08, 0x8E);
    set_gate(37, irq5 as u32, 0x08, 0x8E);
    set_gate(38, irq6 as u32, 0x08, 0x8E);
    set_gate(39, irq7 as u32, 0x08, 0x8E);
    set_gate(40, irq8 as u32, 0x08, 0x8E);
    set_gate(41, irq9 as u32, 0x08, 0x8E);
    set_gate(42, irq10 as u32, 0x08, 0x8E);
    set_gate(43, irq11 as u32, 0x08, 0x8E);
    set_gate(44, irq12 as u32, 0x08, 0x8E);
    set_gate(45, irq13 as u32, 0x08, 0x8E);
    set_gate(46, irq14 as u32, 0x08, 0x8E);
    set_gate(47, irq15 as u32, 0x08, 0x8E);

    set_gate(0x80, isr80 as u32, 0x08, 0xEF);

    debug!(
        "Pointer:",
        Hexadecimal::Unsigned(&IDT_POINTER as *const IDTPointer as usize)
    );
    debug!(
        "Entries:",
        Hexadecimal::Unsigned(&IDT_ENTRIES as *const IDTEntry as usize)
    );
    debug!("Limit:", Hexadecimal::Unsigned(IDT_POINTER.limit as usize));

    idt_flush(&IDT_POINTER as *const IDTPointer);

    core::arch::asm!("sti");

    register_handler(14, page_fault);
}

pub fn page_fault(regs: X86Registers) {
    let cr2: u32;
    unsafe {
        core::arch::asm!("mov {0:e}, cr2", out(reg) cr2);
    }

    debug!(
        "Page fault at address: ",
        Hexadecimal::Unsigned(cr2 as usize)
    );

    loop {}
}

pub unsafe fn set_gate(num: usize, base: u32, selector: u16, flags: u8) {
    IDT_ENTRIES[num].base_low = base as u16 & 0xFFFF;
    IDT_ENTRIES[num].base_high = (base >> 16) as u16 & 0xFFFF as u16;

    IDT_ENTRIES[num].selector = selector;
    IDT_ENTRIES[num].always_zero = 0;

    IDT_ENTRIES[num].flags = flags;
}

static mut INTERRUPT_HANDLERS: [*const u32; 256] = [0 as *const _; 256];

#[no_mangle]
pub unsafe extern "C" fn isr_handler(regs: X86Registers) {
    debug!("Interrupt!");
    let handler = INTERRUPT_HANDLERS[regs.int_num as usize] as *const ();
    if handler != core::ptr::null_mut() {
        let f: fn(X86Registers) = core::mem::transmute(handler);
        f(regs);
    }
}

#[no_mangle]
pub unsafe extern "C" fn irq_handler(regs: X86Registers) {
    if regs.int_num >= 40 {
        out8(0xA0, 0x20);
    }

    out8(0x20, 0x20);

    let handler = INTERRUPT_HANDLERS[regs.int_num as usize];

    if handler != core::ptr::null_mut() {
        let f: fn(X86Registers) = core::mem::transmute(handler);
        f(regs);
    }
}

pub fn register_handler(interrupt_nr: usize, func: fn(X86Registers)) {
    unsafe {
        INTERRUPT_HANDLERS[interrupt_nr] = func as *const u32;
    }
}
