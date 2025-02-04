# Compiler and flags
CC = $(if $(TARGET),$(TARGET)-gcc,gcc)
AS = nasm
LD = i386-elf-ld

# Flags
LDFLAGS = -ffreestanding -nostdlib -nodefaultlibs -lgcc
CFLAGS ?= -O2 -g
CFLAGS := $(CFLAGS) -ffreestanding -Wall -Wextra \
                    -fno-builtin \
                    -fno-exceptions \
                    -fno-stack-protector

# Directories
KERNEL_DIR = src/kernel
ARCH_DIR = src/arch/i386
LIBC_DIR = src/libc
BUILD_DIR = build

# Include directories
INCLUDE_DIRS = -I$(KERNEL_DIR)/include \
               -I$(LIBC_DIR)/include \
               -I$(ARCH_DIR) \
               -I$(KERNEL_DIR)/include/kernel

# Update CFLAGS to include header directories
CFLAGS += $(INCLUDE_DIRS)

# Source files - using find to get all .c files recursively
KERNEL_SRCS = $(wildcard $(KERNEL_DIR)/*.c) \
              $(wildcard $(ARCH_DIR)/*.c)
LIBC_SRCS = $(shell find $(LIBC_DIR) -name '*.c')
ASM_SRCS = $(wildcard $(ARCH_DIR)/*.asm)

# Object files - maintain directory structure in build directory
KERNEL_OBJS = $(KERNEL_SRCS:%.c=$(BUILD_DIR)/%.o)
LIBC_OBJS = $(LIBC_SRCS:%.c=$(BUILD_DIR)/%.o)
ASM_OBJS = $(ASM_SRCS:%.asm=$(BUILD_DIR)/%.o)

# All objects combined
ALL_OBJS = $(KERNEL_OBJS) $(LIBC_OBJS) $(ASM_OBJS)

# The kernel binary
KERNEL = $(BUILD_DIR)/myos.bin

# Build targets
all: $(BUILD_DIR) $(KERNEL)

# Create necessary build directories
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)/$(KERNEL_DIR)
	mkdir -p $(BUILD_DIR)/$(ARCH_DIR)
	mkdir -p $(BUILD_DIR)/$(LIBC_DIR)/stdio
	mkdir -p $(BUILD_DIR)/$(LIBC_DIR)/stdlib
	mkdir -p $(BUILD_DIR)/$(LIBC_DIR)/string

# Link everything together
$(KERNEL): $(ALL_OBJS)
	$(CC) -T $(ARCH_DIR)/linker.ld -o $@ $(LDFLAGS) $^

# Compile C files
$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) -c $< -o $@ $(CFLAGS)

# Compile assembly files
$(BUILD_DIR)/%.o: %.asm
	@mkdir -p $(dir $@)
	$(AS) -f elf32 $< -o $@

# Clean rules
clean:
	rm -rf $(BUILD_DIR)

fclean: clean

re: clean all

bear:
	bear -- make

.PHONY: all clean fclean re bear
