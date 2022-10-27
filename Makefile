TARGET            = aarch64-unknown-none-softfloat
KERNEL_BIN        = kernel8.img
QEMU_BINARY       = qemu-system-aarch64
QEMU_MACHINE_TYPE = raspi3
QEMU_RELEASE_ARGS = -d in_asm -display none
QEMU_RUST_ARGS    = -serial stdio -display none
LD_SCRIPT_PATH    = $(shell pwd)/src/bsp/raspberrypi
RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72
KERNEL_ELF        = target/$(TARGET)/release/kernel
DOCKER_IMAGE      = docker.io/rustembedded/osdev-utils:2021.12

KERNEL_LINKER_SCRIPT = kernel.ld

RUSTFLAGS = $(RUSTC_MISC_ARGS)                   \
    -C link-arg=--library-path=$(LD_SCRIPT_PATH) \
    -C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) \
    -D warnings                   \
    -D missing_docs

COMPILER_ARGS = --target=$(TARGET) \
    --features=bsp-rpi-4           \
    --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)

OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

EXEC_QEMU = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)

DOCKER_CMD          = docker run -t --rm -v $(shell pwd):/work/tutorial -w /work/tutorial $(DOCKER_IMAGE)


qemu:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

	$(call color_header, "Generating stripped binary")
	$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	$(call color_progress_prefix, "Size")
	$(call disk_usage_KiB, $(KERNEL_BIN))

	$(call color_header, "Launching QEMU")
	$(DOCKER_CMD) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

qemu-rust:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

	$(call color_header, "Generating stripped binary")
	$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	$(call color_progress_prefix, "Size")
	$(call disk_usage_KiB, $(KERNEL_BIN))

	$(call color_header, "Launching QEMU")
	$(DOCKER_CMD) $(EXEC_QEMU) $(QEMU_RUST_ARGS) -kernel $(KERNEL_BIN)

clean:
	rm -rf target $(KERNEL_BIN)