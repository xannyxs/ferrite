	;------------------------------------------------------------------------------
	; Boot Assembly Entry Point

	; This file serves as the entry point for our kernel, setting up the minimal
	; environment needed before we can jump to our Rust code. It performs three
	; critical tasks:

	; 1. Creates a multiboot header that marks this as a kernel for GRUB/multiboot
	; bootloaders. The header includes magic numbers and flags that tell the
	; bootloader how to load us.
	; 2. Sets up a small stack (16KB) that our kernel will initially use. The stack
	; is crucial because Rust code requires it for function calls and local variables.

	; 3. Transfers control from assembly to our main kernel code written in Rust
	; ensuring we never return from it (since there's nothing to return to).
	;------------------------------------------------------------------------------

	;        Multiboot header constants
	MBALIGN  equ 1<<0; Align loaded modules on page boundaries
	MEMINFO  equ 1<<1; Provide memory map
	VIDEO    equ 1<<2; Video mode
	MBFLAGS  equ MBALIGN | MEMINFO | VIDEO; Combine our flags
	MAGIC    equ 0x1BADB002; Magic number lets bootloader find the header
	CHECKSUM equ -(MAGIC + MBFLAGS); Checksum required by multiboot standard

	;       First section: Multiboot header
	section .multiboot
	align   4; Header must be 4-byte aligned
	dd      MAGIC; Write the magic number
	dd      MBFLAGS; Write the flags
	dd      CHECKSUM; Write the checksum
	dd      0, 0, 0, 0, 0; Reserved fields
	dd      1; Mode type: 1 means text mode
	dd      80; Width in characters
	dd      25; Height in characters
	dd      0; Depth (0 for text mode)

	; ----------------------------------------------

	;       Second section: Stack setup
	global  stack_bottom
	section .bss
	align   16

stack_bottom:
	;WARNING Do not change value without change it in memory/stack.rs
	resb     16384; Reserve 16KB for our stack.

stack_top:

	; ----------------------------------------------

	section .boot
	global  _start:function

_start:
	mov dx, 0x3F8; COM1 port
	out dx, al

	mov ecx, (initial_page_dir - 0xC0000000)
	mov cr3, ecx

	mov ecx, cr4
	or  ecx, 0x10
	mov cr4, ecx

	mov ecx, cr0
	or  ecx, 0x80000000
	mov cr0, ecx

	jmp higher_half

section .text

higher_half:
	mov esp, stack_top

	mov dx, 0x3F8
	out dx, al

	push ebx
	push eax
	xor  ebp, ebp

	;      Fetch extern functions
	extern gdt_init
	extern idt_init
	extern pic_remap
	extern kernel_main

	;    Init gdt
	call gdt_init

	;    Init idt
	call idt_init

	;    Initiate PIC
	push 28
	push 20
	call pic_remap
	add  esp, 8

	; Activate interrupts
	sti

	;    Call kernel
	call kernel_main

.hang:
	hlt ; Halt the CPU
	jmp .hang

.end:
	global _start.end

	section .data
	align   4096
	global  initial_page_dir

initial_page_dir:
	dd    10000011b
	times (768 - 1) dd 0

	dd    (0 << 22) | 10000011b
	dd    (1 << 22) | 10000011b
	dd    (2 << 22) | 10000011b
	dd    (3 << 22) | 10000011b
	times (1024 - 768 - 4) dd 0
