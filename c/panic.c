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

typedef unsigned int size_t;
typedef int int32_t;

size_t strlen(const char *str){
	if(str == 0) {
		return 0;
	}

    size_t len = 0;
    while (str[len] != 0){
        len++;
    }
    return len;
}

int32_t memcmp(const char *s1, const char *s2, size_t n){
    unsigned char u1, u2;

    for (; n--; s1++, s2++){
        u1 = *(unsigned char *)s1;
        u2 = *(unsigned char *)s2;
        if (u1 != u2){
            return (u1 - u2);
        }
    }
    return 0;
}
