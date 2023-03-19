typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;

const char* panic_message = "[C code] A panic was occured!";

void __outb(uint16_t port, uint8_t val){
    asm volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

void __write_com1_string(const char* str) {
	do {
		__outb(0x3f8, *str);
	} while(*str++);
}

void __panic_handler_c() {
	__write_com1_string(panic_message);

	while(1) {}
}
