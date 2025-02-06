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
	MBFLAGS  equ MBALIGN | MEMINFO; Combine our flags
	MAGIC    equ 0x1BADB002; Magic number lets bootloader find the header
	CHECKSUM equ -(MAGIC + MBFLAGS); Checksum required by multiboot standard

	;       First section: Multiboot header
	section .multiboot
	align   4; Header must be 4-byte aligned
	dd      MAGIC; Write the magic number
	dd      MBFLAGS; Write the flags
	dd      CHECKSUM; Write the checksum

	; ----------------------------------------------

	;       Second section: Stack setup
	section .bss
	align   16; Ensure proper alignment for the stack

stack_bottom:
	resb 16384; Reserve 16KB for our stack

stack_top:

	; ----------------------------------------------

	section .data

gdt_start:
	;  Null descriptor (required)
	dd 0
	dd 0

	;  Code segment descriptor
	dw 0xFFFF; Limit (bits 0-15)
	dw 0; Base (bits 0-15)
	db 0; Base (bits 16-23)
	db 10011010b; Access byte - Present=1, Ring=00, Type=1, Code=1, Conforming=0, Readable=1, Accessed=0
	db 11001111b; Flags and Limit (bits 16-19) - Granularity=1, 32-bit=1, 64-bit=0, AVL=0, Limit=1111b
	db 0; Base (bits 24-31)

	;  Data segment descriptor
	dw 0xFFFF; Limit (bits 0-15)
	dw 0; Base (bits 0-15)
	db 0; Base (bits 16-23)
	db 10010010b; Access byte - Present=1, Ring=00, Type=1, Code=0, Expand Down=0, Writable=1, Accessed=0
	db 11001111b; Flags and Limit (bits 16-19)
	db 0; Base (bits 24-31)

gdt_end:

gdt_descriptor:
	dw gdt_end - gdt_start - 1; Size of GDT
	dd gdt_start; Start address of GDT

	; Define segment selectors
	CODE_SEG equ 0x08  ; First descriptor after null
	DATA_SEG equ 0x10  ; Second descriptor after null

	; ----------------------------------------------

	section .text
	global  _start:function

_start:
	;   Set up stack pointer
	mov esp, stack_top

	;    Load GDT
	lgdt [gdt_descriptor]

	;   Enable protected mode
	mov eax, cr0
	or  eax, 1
	mov cr0, eax

	;   Far jump to set CS register and clear pipeline
	jmp CODE_SEG:start_protected_mode

[bits 32]

start_protected_mode:
	;   Set up segment registers
	mov ax, DATA_SEG
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	;   Now set up the stack in the new segment
	mov esp, stack_top

	extern test_memory_access
	call   test_memory_access

	;      Call kernel
	extern kernel_main
	call   kernel_main

	cli ; Disable interrupts

.hang:
	hlt ; Halt the CPU
	jmp .hang; If we wake up (e.g., from NMI), halt again

.end:
	global _start.end
