.code32
.set ALIGN,		 				1<<0
.set MEMINFO,	 				1<<1
.set VBE_MODE,   				1<<2
.set INIT_MBOOT_HEADER_MAGIC,   0x1BADB002
.set INIT_MBOOT_HEADER_FLAGS,   ALIGN + MEMINFO + VBE_MODE
.set INIT_MBOOT_CHECKSUM,       0x00000000 - (INIT_MBOOT_HEADER_MAGIC + INIT_MBOOT_HEADER_FLAGS)

.set STACK_SIZE, 1024 * 64  # 64 KB


.section .multiboot, "aw"
.align 4
.int INIT_MBOOT_HEADER_MAGIC
.int INIT_MBOOT_HEADER_FLAGS
.int INIT_MBOOT_CHECKSUM
.long 0, 0, 0, 0, 0
.long 0
.long 800, 600, 32      # Width, Height, Depth

# Stack here
.section .bootstrap_stack, "aw", @nobits
    .global stack_bottom
    .align 4
	stack_bottom:
		.skip STACK_SIZE
	stack_top:

# Our page directory and page table
.section .bss, "aw", @nobits
	.align 4096
    .global boot_page_directory
    boot_page_directory:
        .skip 4096
    boot_page_table:
        .skip 4096

.section	.text

.global _init_early

_init_early:
    # Page table - Virtual address = Physical address.
	movl $(boot_page_table - 0xC0000000), %edi

	# Set start point
	movl $0, %esi

	# Count of pages will be mapped
	movl $1024, %ecx

.map_kernel:
	cmpl $0, %esi
    jl .next

	cmpl $(KERNEL_PHYS_END), %esi
	jge .finish_map

	# Map physical address as "present, writable". Note that this maps
	# .text and .rodata as writable. Mind security and map them as non-writable.
	movl %esi, %edx
	orl $0x03, %edx
	movl %edx, (%edi)

.next:
    addl $4096, %esi

    # Size of entries in boot_page_table is 4 bytes.
    addl $4, %edi

    # Loop to the next entry if we haven't finished.
    loop .map_kernel

.finish_map:
    movl $(boot_page_table - 0xC0000000 + 0x003), boot_page_directory - 0xC0000000 + 0
	movl $(boot_page_table - 0xC0000000 + 0x003), boot_page_directory - 0xC0000000 + 768 * 4

	# Recursive paging
	movl $(boot_page_directory - 0xC0000000 + 3), boot_page_directory - 0xC0000000 + 4092

	# Set cr3 to the address of the boot_page_directory.
	movl $(boot_page_directory - 0xC0000000), %ecx
	movl %ecx, %cr3

	# Enable paging and the Write-protect bit.
	movl %cr0, %ecx
	orl $0x80010000, %ecx
	movl %ecx, %cr0

	# Jump to higher half with an absolute jump.
	lea .finally, %ecx
    jmp *%ecx

.finally:
    # Unmap our table
	movl $0, boot_page_directory

	# Reload cr3 to force a TLB flush so the changes to take effect.
	movl %cr3, %ecx
	movl %ecx, %cr3

    
    # Setup our stack
    mov $stack_top, %esp

    # TODO: Initialize SSE, FPU

    # First argument is pointer to multiboot header
    # Second argument is address of our stack

    mov $boot_page_directory, %ecx
    push %ecx 
    
    mov $stack_bottom, %ecx
    push %ecx

    push %esp
    

    addl $0xC0000000, %ebx
    push %ebx  # Grub puts 0x10000 here

    xor %ebp, %ebp

    # Jump to our kernel entry
    call _start

    cli
    hlt

    1:
        jmp 1b
