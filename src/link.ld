ENTRY(_init_early)

SECTIONS
{
    . = 0x00200000;
    KERNEL_BASE_pos = .;

    .text ALIGN (0x1000) :
    {
        *(.boot)
        *(.text)
    }

    .data ALIGN (0x1000) :
    {
        *(.data)
    }

    .bss :
    {
        *(COMMON)
        *(.bss)
    }

	/DISCARD/ : {
		*(.note.gnu.build-id)
	}

    KERNEL_END_pos = .;
}
