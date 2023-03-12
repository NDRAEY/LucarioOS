include include.mk

all: $(NASM)
	@rustup override set nightly

	@cargo rustc --release --target x86_64-unknown-none -- --emit=obj

	$(MAKE) -f Makefile_Stage2

$(NASM): asm/%.o : asm/%.asm
	@echo -e '\x1b[32mASM  \x1b[0m' $@
	@$(AS) $< -o $@

iso:
	$(MAKE) -f Makefile_Stage2 iso

run:
	$(MAKE) -f Makefile_Stage2 run

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