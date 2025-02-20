BIN=$(KERNEL_DIR)/fungul.bin
ISO=$(KERNEL_DIR)/kernel.iso

## Add in CLI --release to compile production code
CARGO_FLAGS ?= 

KERNEL_DIR = src
ARCH_DIR = src/arch/x86
KERNEL_DIR = src/kernel
LIBC_DIR = src/libc

all:
	cargo build --manifest-path=src/sync/Cargo.toml $(CARGO_FLAGS)
	cargo build --manifest-path=src/kernel/Cargo.toml $(CARGO_FLAGS)

clean:
	cargo clean --manifest-path=src/kernel/Cargo.toml
	cargo clean --manifest-path=src/sync/Cargo.toml

fclean:
	rm -f $(ISO)
	rm -rf $(KERNEL_DIR)/isodir
	cargo clean --manifest-path=src/kernel/Cargo.toml
	cargo clean --manifest-path=src/sync/Cargo.toml

re: fclean all

run: all
	cargo run --manifest-path=src/kernel/Cargo.toml

.PHONY: all clean fclean re run 
