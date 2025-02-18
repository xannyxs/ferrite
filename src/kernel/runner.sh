#!/bin/sh

# Build iso

mkdir -p isodir/boot/grub
cp "$1" isodir/boot/fungul.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o kernel.iso isodir

# Run the kernel

export QEMUFLAGS="-device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -display none"

qemu-system-i386 -cdrom kernel.iso $QEMUFLAGS
EXIT=$?

if [ "$EXIT" -ne 33 ]; then
	exit 1
fi
