#!/bin/sh

# Build iso

mkdir -p isodir/boot/grub
cp "$1" isodir/boot/ferrite.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o kernel.iso isodir

# Set QEMU flags based on the command
if [ "$2" = "test" ]; then
    # For cargo test, we need the extra flags for test handling
    export QEMUFLAGS="-device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -display none"
elif [ "$2" = "debug" ]; then
    export QEMUFLAGS="-serial stdio -s -S"
else
    # For cargo run, we just need basic serial output
    export QEMUFLAGS="-serial stdio"
fi

qemu-system-i386 -cdrom kernel.iso $QEMUFLAGS
EXIT=$?

if [ "$EXIT" -ne 33 ]; then
	exit 1
fi
