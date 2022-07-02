boot_source_files := $(shell find boot -name *.asm)
boot_object_files := $(patsubst boot/%.asm, build/%.o, $(boot_source_files))

$(boot_object_files): build/%.o : boot/%.asm Makefile
	mkdir -p $(dir $@)
	nasm -f elf64 $(patsubst build/%.o, boot/%.asm, $@) -o $@

#

build/libfe2o3.a: Cargo.lock Cargo.toml src/** Makefile
	mkdir -p build
	cargo build --release --target-dir=build/target/
	cp build/target/x86_64-unknown-none/release/libfe2o3.a $@

build/kernel.bin: linker.ld $(boot_object_files) build/libfe2o3.a Makefile
	mkdir -p build
	ld -m elf_x86_64 --gc-sections -T $< -o $@ $(boot_object_files) build/libfe2o3.a 

build/fe2o3.iso: build/kernel.bin Makefile
	cp $< iso/boot/kernel.bin
	grub-mkrescue /usr/lib/grub/i386-pc -o $@ iso

#

build/libfe2o3-tests.a: Cargo.lock Cargo.toml src/** Makefile
	mkdir -p build
	cargo build --release --target-dir=build/target-tests/ --features=tests
	cp build/target-tests/x86_64-unknown-none/release/libfe2o3.a $@

build/kernel-tests.bin: linker.ld $(boot_object_files) build/libfe2o3-tests.a Makefile
	mkdir -p build
	ld -m elf_x86_64 --gc-sections -T $< -o $@ $(boot_object_files) build/libfe2o3-tests.a

build/fe2o3-tests.iso: build/kernel-tests.bin Makefile
	cp $< iso/boot/kernel.bin
	grub-mkrescue /usr/lib/grub/i386-pc -o $@ iso

#

.PHONY: run
run: build/fe2o3.iso Makefile
	qemu-system-x86_64 $<

.PHONY: test
test: build/fe2o3-tests.iso Makefile
	qemu-system-x86_64 $< -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -display none --no-reboot

.PHONY: clean
clean:
	rm -rf build/
	rm -rf target/