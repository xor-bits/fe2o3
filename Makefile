KERNEL_LIB      = target/boot/libfe2o3.a
KERNEL_BIN      = target/boot/kernel.bin
KERNEL_LIB_DEPS = $(filter-out %: ,$(file < target/x86_64-unknown-none/release/libfe2o3.d)) Cargo.toml Cargo.lock
OS_ISO          = target/boot/fe2o3.iso
LD_SCRIPT       = linker.ld

BOOT_SOURCE     = $(shell find src/boot -name *.asm)
BOOT_OBJECT     = $(patsubst src/boot/%.asm, target/boot/%.o, $(BOOT_SOURCE))



.PHONY: run clean

# compile assembly
$(BOOT_OBJECT): $(BOOT_SOURCE) Makefile
	@mkdir -p $(dir $@)
	@nasm -f elf64 $(patsubst target/boot/%.o, src/boot/%.asm, $@) -o $@

# compile rust
$(KERNEL_LIB): $(KERNEL_LIB_DEPS) Makefile
	@echo "Compile kernel library"
	@mkdir -p $(shell dirname $(KERNEL_BIN))
	@cargo build --release
	@cp target/x86_64-unknown-none/release/libfe2o3.a $@

# link
$(KERNEL_BIN): $(KERNEL_LIB) $(BOOT_OBJECT) $(LD_SCRIPT) Makefile
	@echo "Compile kernel binary"
	@ld -m elf_x86_64 --gc-sections -T $(LD_SCRIPT) -o $@ $(BOOT_OBJECT) $(KERNEL_LIB)

# generate iso
$(OS_ISO): $(KERNEL_BIN) Makefile
	@echo "Generate os iso"
	@cp $(KERNEL_BIN) iso/boot/kernel.bin
	@grub-mkrescue /usr/lib/grub/i386-pc -o $@ iso

# acts

run: $(OS_ISO) Makefile
	@qemu-system-x86_64 $<

clean:
	@rm -rf target/
