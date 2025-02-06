BIN=rust_kfs.bin
ISO=rust_kfs.iso
KERNEL=libkernel.a

AS = nasm
ASFLAGS=-felf32

## Add in CLI --release to compile production code
CARGO_FLAGS ?= 

KERNEL_DIR = src/kernel
ARCH_DIR = src/arch/x86
LIBC_DIR = src/libc

ASM_TARGET_DIR = target/asm/x86
BUILD_DIR = target/build

LD=ld
LDFLAGS=-m elf_i386 --no-dynamic-linker -static

ASM_SRCS = $(shell find $(ARCH_DIR) -name '*.asm')
ASM_OBJS = $(patsubst $(ARCH_DIR)/%.asm,$(ASM_TARGET_DIR)/%.o,$(ASM_SRCS))


all: $(BIN)

$(BIN): $(KERNEL) $(ASM_OBJS)
	$(LD) $(LDFLAGS) -T $(ARCH_DIR)/x86.ld -o $(BIN) $(ASM_OBJS) target/x86/debug/$(KERNEL)

$(KERNEL):
	cargo build $(CARGO_FLAGS)

$(ASM_TARGET_DIR)/%.o: $(ARCH_DIR)/%.asm
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

clean:
	cargo clean

fclean:
	rm -f $(BIN)
	rm -f $(ISO)
	cargo clean

re: fclean all

run: $(BIN)
	qemu-system-i386 -kernel $(BIN) -no-reboot

run_debug: $(BIN)
	qemu-system-i386 -kernel $(BIN) -no-reboot -serial stdio -d int -D qemu.log

iso:
	mkdir -p isodir/boot/grub
	cp rust_kfs.bin isodir/boot/rust_kfs.bin
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) isodir
	qemu-system-i386 -cdrom $(ISO) -no-reboot

.PHONY: all clean fclean re run run_debug iso
