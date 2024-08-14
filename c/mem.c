extern unsigned int KERNEL_VIRT_START;
extern unsigned int KERNEL_VIRT_END;

extern unsigned int KERNEL_PHYS_START;
extern unsigned int KERNEL_PHYS_END;


unsigned int get_kernel_virt(unsigned int* start, unsigned int* end) {
    *start = (unsigned int)(&KERNEL_VIRT_START);
    *end = (unsigned int)(&KERNEL_VIRT_END);
}
