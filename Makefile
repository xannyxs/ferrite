BIN=$(KERNEL_DIR)/fungul.bin
ISO=$(KERNEL_DIR)/kernel.iso

## Add in CLI --release to compile production code
CARGO_FLAGS ?= 

KERNEL_DIR = src/kernel

all:
	cargo build --manifest-path=src/kernel/Cargo.toml $(CARGO_FLAGS)

clean:
	cargo clean --manifest-path=src/kernel/Cargo.toml

fclean:
	rm -f $(ISO)
	rm -rf $(KERNEL_DIR)/isodir
	cargo clean --manifest-path=src/kernel/Cargo.toml

re: fclean all

run: all
	cd $(KERNEL_DIR) && cargo run

test: all
	cd $(KERNEL_DIR) && cargo ltest

debug: all
	cd $(KERNEL_DIR) && cargo debug 

.PHONY: all clean fclean re run debug
