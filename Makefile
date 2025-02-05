QEMU=qemu-system-i386
CARGO=cargo

NAME=kfs.elf

AS=nasm
ASFLAGS=-felf32

LD=ld
LDFLAGS=-n -nostdlib -m elf_i386

KLIB=libkfs.a # TODO debug ? release ?


all: $(NAME)

$(NAME): $(KLIB) asm link

$(KLIB):
	$(CARGO) build --target "src/arch/x86/x86.json"

asm:
	$(AS) $(ASFLAGS) src/arch/x86/boot.asm -o target/boot.o

link:
	$(LD) $(LDFLAGS) -T src/arch/x86/x86.ld -o $(NAME) target/boot.o target/x86/debug/$(KLIB) # TODO debug ?

clean:
	$(CARGO) clean
	rm -f $(NAME)

run: $(NAME)
	$(QEMU) -kernel $(NAME)

run_debug: $(NAME)
	$(QEMU) -kernel $(NAME) -no-reboot -serial stdio -d int -D qemu.log
