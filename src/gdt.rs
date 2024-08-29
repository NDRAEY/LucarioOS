use core::mem::size_of;

#[repr(C, packed)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GDTPointer {
    limit: u16,
    base: *const GDTEntry,
}

impl GDTEntry {
    pub const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> GDTEntry {
        Self {
            limit_low: (limit & 0xffff) as u16,
            granularity: ((limit >> 16) & 0xf) as u8 | (granularity & 0xf0),

            base_low: (base & 0xffff) as u16,
            base_middle: ((base >> 16) & 0xff) as u8,
            base_high: ((base >> 24) & 0xff) as u8,

            access,
        }
    }
}

static mut GDT_REAL: [GDTEntry; 5] = [
    GDTEntry::new(0, 0, 0, 0), // Null always
    GDTEntry::new(0, 0xffff_ffff, 0x9a, 0xcf),
    GDTEntry::new(0, 0xffff_ffff, 0x92, 0xcf),
    GDTEntry::new(0, 0xffff_ffff, 0xfa, 0xcf),
    GDTEntry::new(0, 0xffff_ffff, 0xf2, 0xcf),
];

static mut GDT_POINTER: GDTPointer = GDTPointer {
    limit: 0,
    base: core::ptr::null(),
};

use crate::{debug, log::log::DebugWrite};

extern "C" {
    fn gdt_flush(a: *const GDTPointer);
}

pub unsafe fn init() {
    GDT_POINTER = GDTPointer {
        limit: (GDT_REAL.len() * size_of::<GDTEntry>() - 1) as u16,
        base: &GDT_REAL as *const GDTEntry,
    };

    gdt_flush(&GDT_POINTER);

    debug!("GDT initialized!");
}
