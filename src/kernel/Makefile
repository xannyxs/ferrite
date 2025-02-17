BIN=fungul.bin
ISO=fungul.iso

## Add in CLI --release to compile production code
CARGO_FLAGS ?= 

KERNEL_DIR = src
ARCH_DIR = ../arch/x86
LIBC_DIR = src/libc

all:
	cargo build $(CARGO_FLAGS)

clean:
	cargo clean

fclean:
	rm -f $(BIN)
	rm -f $(ISO)
	cargo clean

re: fclean all

run: $(BIN)
	qemu-system-i386 -kernel $(BIN)

run_debug: $(BIN)
	qemu-system-i386 -kernel $(BIN) -no-reboot -serial stdio -d int -D qemu.log

iso:
	mkdir -p isodir/boot/grub
	cp target/x86/debug/fungul isodir/boot/rust_kfs.bin
	cp grub.cfg isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) isodir
	qemu-system-i386 -cdrom $(ISO)

.PHONY: all clean fclean re run run_debug iso 
