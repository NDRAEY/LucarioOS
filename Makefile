include include.mk

TARGET = x86_64-unknown-none

KERNEL = LucarioOS.elf

DEBUG ?= false

ifeq ($(DEBUG),true)
	CARGO_DEBUG = 
	DEPS = target/$(TARGET)/debug/deps/
else
	CARGO_DEBUG = --release
	DEPS = target/$(TARGET)/release/deps/
endif

all: $(KERNEL)

$(KERNEL): Cargo.toml src/*.rs src*/*.rs $(NASM)
	@rustup override set nightly
	@rustup target add x86_64-unknown-none

	@cargo rustc $(CARGO_DEBUG) --target $(TARGET) -- \
				--emit=obj \
				-C panic=abort \
				-C default-linker-libraries=false

	$(LD) -n $(DEPS)/*.o $(NASM) \
		-T src/link.ld \
		-o $(KERNEL)

$(NASM): asm/%.o : asm/%.asm
	@echo -e '\x1b[32mASM  \x1b[0m' $@
	@$(AS) $< -o $@

iso: $(KERNEL)
	-mkdir -p isodir/boot/grub
	mv $(KERNEL) isodir/boot/
	cp src/grub.cfg isodir/boot/grub

	grub-mkrescue isodir/ -o LucarioOS.iso -V LucarioOS

run:
	qemu-system-x86_64 -enable-kvm -m 16M -serial mon:stdio -cdrom LucarioOS.iso

runiso:
	$(MAKE) iso
	$(MAKE) run

everything:
	$(MAKE)
	$(MAKE) runiso

clean:
	-rm $(NASM)
	-rm isodir -rf
	-rm target -rf
