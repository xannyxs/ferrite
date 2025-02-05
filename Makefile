NAME=rust_kfs.bin
KERNEL=libkernel.a

AS = nasm
ASFLAGS=-felf32

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


all: $(NAME)

$(NAME): $(KERNEL) $(ASM_OBJS)
	$(LD) $(LDFLAGS) -T $(ARCH_DIR)/x86.ld -o $(NAME) $(ASM_OBJS) target/x86/debug/$(KERNEL)

$(KERNEL):
	cargo build $(CARGO_FLAGS)

$(ASM_TARGET_DIR)/%.o: $(ARCH_DIR)/%.asm
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

clean:
	cargo clean

fclean:
	rm -f $(NAME)
	cargo clean

re: fclean all

run: $(NAME)
	qemu-system-i386 -kernel $(NAME) -no-reboot

run_debug: $(NAME)
	qemu-system-i386 -kernel $(NAME) -no-reboot -serial stdio -d int -D qemu.log
