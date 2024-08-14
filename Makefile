include include.mk

TARGET = i686.json

KERNEL = LucarioOS.elf

LD = ld.lld

RUSTUP = rustup
CARGO = cargo

DEBUG ?= true

ifeq ($(DEBUG),true)
	CARGO_DEBUG = 
	# DEPS = target/$(TARGET)/debug/deps/
	DEPS = target/i686/debug/deps/
else
	CARGO_DEBUG = --release
	# DEPS = target/$(TARGET)/release/deps/
	DEPS = target/i686/release/deps/
endif

all: $(KERNEL)

TOOLCHAIN = nightly-2023-04-11

$(KERNEL): Cargo.toml src/*.rs src/*/*.rs $(C_OBJS) $(ASM)
	@$(RUSTUP) override set $(TOOLCHAIN)
	rustup component add rust-src --toolchain $(TOOLCHAIN)
	@$(RUSTUP) target add x86_64-unknown-none

	$(CARGO) rustc $(CARGO_DEBUG) --target $(TARGET) -Zbuild-std -- \
				--emit=obj \
				-C panic=abort \
				-C overflow-checks=off \
				-C default-linker-libraries=false \
				-C link-args='-Tsrc/link.ld'

	$(LD) -n $(DEPS)/*.o $(DEPS)/*.rlib $(ASM) $(C_OBJS) \
		-T src/link.ld \
		-o $(KERNEL)

$(ASM): asm/%.o : asm/%.S
	@echo -e '\x1b[32mASM  \x1b[0m' $@
	@$(AS) $< --32 -o $@

$(C_OBJS): c/%.o : c/%.c
	@echo -e '\x1b[32mC  \x1b[0m' $@
	@$(CC) $< -ffreestanding -mno-sse -mno-avx -nostdlib -fno-builtin -fno-stack-protector -m32 -c -o $@

iso: $(KERNEL)
	-mkdir -p isodir/boot/grub
	mv $(KERNEL) isodir/boot/
	cp src/grub.cfg isodir/boot/grub

	grub-mkrescue isodir/ -o LucarioOS.iso -V LucarioOS

run:
	qemu-system-x86_64 -d int,guest_errors -m 32M -serial mon:stdio -cdrom LucarioOS.iso -no-reboot

runiso:
	$(MAKE) iso
	$(MAKE) run

everything:
	$(MAKE)
	$(MAKE) runiso

clean:
	-rm $(ASM)
	-rm $(KERNEL)
	-rm $(C_OBJS)
	-rm isodir -rf
	-rm target -rf
