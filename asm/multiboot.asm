.code32

.set ALIGN,		 						1<<0
.set MEMINFO,	 						1<<1
.set VBE_MODE,   						1<<2
.set INIT_MBOOT_HEADER_MAGIC,           0x1BADB002
.set INIT_MBOOT_HEADER_FLAGS,           ALIGN | MEMINFO | VBE_MODE
.set INIT_MBOOT_CHECKSUM,               -(INIT_MBOOT_HEADER_MAGIC + INIT_MBOOT_HEADER_FLAGS)
.set STACK_SIZE, 1024 * 32  # 32 KB

.extern kernel

.section .boot

.int INIT_MBOOT_HEADER_MAGIC
.int INIT_MBOOT_HEADER_FLAGS
.int INIT_MBOOT_CHECKSUM
.long 0, 0, 0, 0, 0
.long 0
.long 800, 600, 32

.section .bss
	.align 16
	stack_bottom:
		.skip STACK_SIZE
	stack_top:

.section	.text
.global		_init_early

_init_early:
		cli 

		# init FPU
		fninit
		fldcw (__fpu_control_word)

		# enable SSE
		mov %cr0, %eax
		and $~0x04, %al
		or $0x22, %al
		mov %eax, %cr0
		
		mov %cr4, %eax
		or $0x600, %ax
		mov %eax, %cr4

		mov $stack_top, %esp

		push	%esp
		push	%ebx

		call	_start

		hlt

__fpu_control_word:
		.word 0x37f

loop:
		jmp	loop
