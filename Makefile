# Compiler and flags
CC = $(if $(TARGET),$(TARGET)-gcc,gcc)
AS = nasm
LD = i386-elf-ld

# Flags
LDFLAGS = -ffreestanding -nostdlib -nodefaultlibs -lgcc
CFLAGS = -ffreestanding -O2 -Wall -Wextra \
				 -fno-builtin \
				 -fno-exceptions \
				 -fno-stack-protector

# Directories
KERNEL_DIR = src/kernel
ARCH_DIR = src/arch/i386
BUILD_DIR = build

# Source files
KERNEL_SRCS = $(wildcard $(KERNEL_DIR)/*.c)
ASM_SRCS = $(wildcard $(ARCH_DIR)/*.asm)

# Object files
KERNEL_OBJS = $(KERNEL_SRCS:$(KERNEL_DIR)/%.c=$(BUILD_DIR)/%.o)
ASM_OBJS = $(ASM_SRCS:$(ARCH_DIR)/%.asm=$(BUILD_DIR)/%.o)

# The kernel binary
KERNEL = $(BUILD_DIR)/myos.bin

all: $(BUILD_DIR) $(KERNEL)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(KERNEL): $(KERNEL_OBJS) $(ASM_OBJS)
	$(CC) -T $(ARCH_DIR)/linker.ld -o $@ $(LDFLAGS) $^

$(BUILD_DIR)/%.o: $(KERNEL_DIR)/%.c
	$(CC) -c $< -o $@ $(CFLAGS)

$(BUILD_DIR)/%.o: $(ARCH_DIR)/%.asm
	$(AS) -f elf32 $< -o $@

compile_commands.json: Makefile
	bear -- make clean all

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all clean
