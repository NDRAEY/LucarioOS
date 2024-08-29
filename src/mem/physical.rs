use core::arch::asm;
use core::mem::size_of;

use crate::conv::fmt::Hexadecimal;
use crate::multiboot::*;
use crate::stubs::mem::memset;

use crate::log::log::DebugWrite;

#[macro_use]
use crate::debug;

const BITMAP_SIZE: usize = 512 << 10; // 512 KB is enough to store info about 4 GB of space
const PAGE_SIZE: usize = 4096;
const PAGE_DIRECTORY_START: usize = 0xffffffff - (4 << 20) + 1;

const PAGE_PRESENT: usize = 1 << 0;
const PAGE_WRITEABLE: usize = 1 << 1;
const PAGE_USER: usize = 1 << 2;
const PAGE_WRITE_THROUGH: usize = 1 << 3;
const PAGE_CACHE_DISABLE: usize = 1 << 4;
const PAGE_ACCESSED: usize = 1 << 5;
const PAGE_DIRTY: usize = 1 << 6;
const PAGE_GLOBAL: usize = 1 << 8;

type PhysicalAddress = usize;
type VirtualAddress = usize;

pub struct PhysicalMemoryManager<'a> {
    initialized: bool,
    bitmap: &'a mut [u8],
    used: usize,
    available: usize,
    kernel_page_directory: Option<*mut u32>,
}

extern "C" {
    static KERNEL_PHYS_START: u32;
    static KERNEL_PHYS_END: u32;
    static boot_page_directory: u32;
}

static mut PHYS_MEM_MANAGER: Option<PhysicalMemoryManager> = None;
static mut BITMAP: [u8; BITMAP_SIZE] = [0; BITMAP_SIZE];

pub unsafe fn ginit(mb: *const MultibootHeader) {
    PHYS_MEM_MANAGER = Some(PhysicalMemoryManager::new(&*mb));

    PHYS_MEM_MANAGER.as_mut().unwrap().init(
        ((*mb).mods_addr + 0xC000_0000) as *const MultibootModListEntry,
        (*mb).mods_count as usize,
    );
}

impl PhysicalMemoryManager<'_> {
    pub fn new(mb: &MultibootHeader) -> Self {
        debug!("Initialization!");

        debug!(mb.mem_upper as usize);

        let memsize = unsafe {
            Self::scan_memory_map(
                (mb.mmap_addr + 0xC000_0000) as *const MemoryMapEntry,
                mb.mmap_length,
            )
        };

        debug!("Found", memsize >> 10, "KB of memory!");

        PhysicalMemoryManager {
            initialized: false,
            bitmap: unsafe { &mut BITMAP },
            used: 0,
            available: memsize,
            kernel_page_directory: None,
        }
    }

    pub unsafe fn init(&mut self, modinfo: *const MultibootModListEntry, length: usize) {
        debug!("LEN:", length);

        let lx = &KERNEL_PHYS_END as *const _ as u32;

        debug!("Addr:", Hexadecimal::Unsigned(lx as usize));

        let suitable_addr = if length == 0 {
            lx
        } else {
            modinfo.add(length - 1).read().mod_end
        };

        debug!("ADDR:", Hexadecimal::Unsigned(suitable_addr as usize));

        let start = &KERNEL_PHYS_START as *const _ as u32;
        let mut start_addr = 0xC000_0000 + start;

        while start_addr <= 0xC000_0000 + lx {
            self.mark_page(start_addr as usize, true);
            //debug!("Marked:", Hexadecimal::Unsigned(start_addr as usize));
            start_addr += PAGE_SIZE as u32;
        }

        debug!(Hexadecimal::Unsigned(self.alloc_page()));
        debug!(Hexadecimal::Unsigned(self.alloc_page()));
        debug!(Hexadecimal::Unsigned(self.alloc_page()));
        debug!(Hexadecimal::Unsigned(self.alloc_page()));
        debug!(Hexadecimal::Unsigned(self.alloc_page()));
        debug!(Hexadecimal::Unsigned(self.alloc_page()));
        debug!(Hexadecimal::Unsigned(self.alloc_page()));

        let pdaddr = &boot_page_directory as *const _ as u32;
        self.kernel_page_directory = Some(pdaddr as *mut u32);
    }

    unsafe fn scan_memory_map(memmap: *const MemoryMapEntry, length: u32) -> usize {
        let mut memory_found: usize = 0;
        let entry_size: u32 = size_of::<MemoryMapEntry>() as u32;
        let entry_count = length / entry_size;

        debug!("Len:", length, "Size:", entry_size);

        for idx in 0..=entry_count {
            let entry = memmap.add(idx as _).read();
            debug!(
                Hexadecimal::Unsigned(entry.addr_low as usize),
                " - size ",
                Hexadecimal::Unsigned(entry.len_low as usize),
                " bytes"
            );

            if entry.type_ == 1 {
                memory_found += entry.len_low as usize;
            }
        }

        memory_found
    }

    pub fn available(&self) -> usize {
        self.available
    }

    pub fn used(&self) -> usize {
        self.used
    }

    pub fn alloc_page(&mut self) -> PhysicalAddress {
        for i in 0..BITMAP_SIZE {
            let map_element = self.bitmap[i];

            if map_element == 0xff {
                continue;
            }

            for j in 0..8 {
                if ((map_element >> j) & 1) == 0 {
                    self.bitmap[i] |= 1 << j;

                    self.used += PAGE_SIZE;

                    return (PAGE_SIZE * 8 * i) + (j * PAGE_SIZE);
                }
            }
        }

        return 0;
    }

    pub fn alloc_pages(&mut self, count: usize) -> PhysicalAddress {
        let mut counter = 0;
        let mut addr = 0;

        let mut si = 0;
        let mut sj = 0;

        for i in 0..BITMAP_SIZE {
            let map_element = self.bitmap[i];

            if map_element == 0xff {
                continue;
            }

            for j in 0..8 {
                if ((map_element >> j) & 1) == 0 {
                    if counter == 0 {
                        si = i;
                        sj = j;

                        addr = (PAGE_SIZE * 8 * i) + (j * PAGE_SIZE);
                    } else if counter == count {
                        while si < BITMAP_SIZE {
                            while sj < 8 {
                                self.bitmap[si] |= 1 << sj;

                                if counter == 0 {
                                    self.used += PAGE_SIZE * count;

                                    return addr;
                                }
                                counter -= 1;

                                sj += 1;
                            }

                            sj = 0;
                            si += 1;
                        }
                    }

                    counter += 1;
                } else {
                    counter = 0;
                    addr = 0;
                }
            }
        }

        return 0;
    }

    pub fn free_page(&mut self, addr: PhysicalAddress) {
        if (addr == 0) {
            return;
        }

        let i = addr / (PAGE_SIZE * 8);
        let j = (addr % (PAGE_SIZE * 8)) / PAGE_SIZE;

        self.bitmap[i] &= !(1 << j);

        self.used -= PAGE_SIZE;
    }

    pub fn free_pages(&mut self, addr: PhysicalAddress, mut count: usize) {
        if addr == 0 {
            return;
        }

        let i = addr / (PAGE_SIZE * 8);
        let j = (addr % (PAGE_SIZE * 8)) / PAGE_SIZE;

        for i in i..BITMAP_SIZE {
            for j in j..8 {
                self.bitmap[i] &= !(1 << j);

                if count == 0 {
                    return;
                }

                count -= 1;
            }
        }

        self.used -= PAGE_SIZE * count;
    }

    pub fn is_used_page(&self, addr: PhysicalAddress) -> bool {
        if addr == 0 {
            return true;
        }

        let i = addr / (PAGE_SIZE * 8);
        let j = (addr % (PAGE_SIZE * 8)) / PAGE_SIZE;

        // Just clear a nth bit
        return ((self.bitmap[i] >> j) & 1) != 0;
    }

    pub fn mark_page(&mut self, addr: PhysicalAddress, used: bool) {
        if addr == 0 {
            return;
        }

        let i = addr / (PAGE_SIZE * 8);
        let j = (addr % (PAGE_SIZE * 8)) / PAGE_SIZE;

        if used {
            self.bitmap[i] |= (1 << j);
        } else {
            self.bitmap[i] &= !(1 << j);
        }
    }

    pub unsafe fn new_page_directory(&mut self) -> *mut u32 {
        let dir: *mut u32 = self.alloc_page() as *mut u32;

        memset(dir as *mut u8, 0, PAGE_SIZE);

        dir.add(1023).write((dir as u32 | 3) as u32); // Cyclic paging

        return dir;
    }

    #[inline]
    fn pd_index(addr: VirtualAddress) -> usize {
        addr >> 22
    }

    #[inline]
    fn pt_index(addr: VirtualAddress) -> usize {
        (addr >> 12) & 0x3ff
    }

    pub unsafe fn get_page_table(&self, page_dir: *mut u32, addr: VirtualAddress) -> *mut u32 {
        let pt = if !self.initialized {
            PAGE_DIRECTORY_START as u32 + (PhysicalMemoryManager::pd_index(addr) * PAGE_SIZE) as u32
        } else {
            page_dir.add(PhysicalMemoryManager::pd_index(addr)).read() & !0xfff
        };

        pt as *mut u32
    }

    #[inline]
    pub unsafe fn reload_cr3() {
        asm!("mov %cr3, %eax\nmov %eax, %cr3");
    }

    pub unsafe fn map_page(
        &mut self,
        mut phys: PhysicalAddress,
        mut virt: VirtualAddress,
        flags: usize,
    ) {
        phys &= !0xfff;
        virt &= !0xfff;

        let pdi = PhysicalMemoryManager::pd_index(virt);
        let pti = PhysicalMemoryManager::pt_index(virt);

        let page_directory = self.kernel_page_directory.unwrap();

        let page_table = if (page_directory.add(pdi).read() & 1) == 0 {
            let mut pt = self.alloc_page();

            page_directory
                .add(pdi)
                .write((pt | PAGE_WRITEABLE | PAGE_PRESENT) as u32);

            if self.initialized {
                pt = PAGE_DIRECTORY_START + (pdi * PAGE_SIZE);

                memset(pt as _, 0, PAGE_SIZE);
            } else {
                memset(pt as _, 0, PAGE_SIZE);
            }

            pt as *mut u32
        } else {
            self.get_page_table(self.kernel_page_directory.unwrap(), virt)
        };

        page_table
            .add(pti)
            .write((phys | flags | PAGE_PRESENT) as u32);

        Self::reload_cr3();
    }

    pub unsafe fn unmap_page(&mut self, mut addr: VirtualAddress) {
        addr &= !0xfff;

        if (self
            .kernel_page_directory
            .unwrap()
            .add(Self::pd_index(addr))
            .read()
            & 1)
            != 0
        {
            let pt = self.get_page_table(self.kernel_page_directory.unwrap(), addr);

            pt.add(Self::pt_index(addr)).write(0);

            Self::reload_cr3();
        }
    }

    pub unsafe fn virt2phys(&self, mut addr: VirtualAddress) -> PhysicalAddress {
        addr &= !0xfff;

        if (self
            .kernel_page_directory
            .unwrap()
            .add(Self::pd_index(addr))
            .read()
            & 1)
            == 0
        {
            return 0;
        } else {
            let pt = self.get_page_table(self.kernel_page_directory.unwrap(), addr);

            return pt.add(Self::pt_index(addr)) as PhysicalAddress & !0xfff;
        }
    }

    pub unsafe fn map_pages(
        &mut self,
        mut phys: PhysicalAddress,
        mut virt: VirtualAddress,
        size: usize,
        flags: usize,
    ) {
        let vend = crate::stubs::align::align(virt + size, PAGE_SIZE);

        while virt <= vend {
            self.map_page(phys, virt, flags);

            phys += PAGE_SIZE;
            virt += PAGE_SIZE;
        }
    }
}
