pub struct MultibootHeader {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u32,
    pub vbe_interface_seg: u32,
    pub vbe_interface_off: u32,
    pub vbe_interface_len: u32,
  
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8
}

pub struct MultibootModList {
	mod_start: u32,
	mod_end: u32,
	cmdline: u32
}

pub struct MemoryMapEntry {
  	size: u32,
  	addr_low: u32,
  	addr_high: u32,
  	len_low: u32,
  	len_high: u32,
  	type_: u32
}