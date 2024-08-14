ENTRY(_init_early);

SECTIONS {
    . = 0x100000;

    KERNEL_PHYS_START = .;
    KERNEL_VIRT_START = . + 0xC0000000;

    .multiboot : {
        *(.multiboot*)
    }

    . += 0xC0000000;

    .text ALIGN(0x1000) : AT(ADDR (.text) - 0xC0000000) {
        *(.text*)
    }

    .data ALIGN(0x1000) : AT(ADDR (.data) - 0xC0000000) {
        *(.data*)
    }

    .rodata ALIGN(0x1000) : AT(ADDR (.rodata) - 0xC0000000) {
        *(.rodata*)
    }

    .bss ALIGN(0x1000) : AT(ADDR (.bss) - 0xC0000000) {
        *(COMMON)
        *(.bss .bss.*)
		/**(.bootstrap_stack)*/
    }

    .bootstrap_stack ALIGN(0x1000) : AT(ADDR(.bootstrap_stack) - 0xC0000000) {
        *(.bootstrap_stack)
    }

    KERNEL_PHYS_END = . - 0xC0000000;
    KERNEL_VIRT_END = .;
}
