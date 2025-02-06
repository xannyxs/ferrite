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

	;       Second section: Stack setup
	section .bss
	align   16; Ensure proper alignment for the stack

stack_bottom:
	resb 16384; Reserve 16KB for our stack

stack_top:

	;       Third section: Entry point code
	section .text
	global  _start:function (_start.end - _start); Export our entry point

_start:
	;   Set up the stack pointer
	mov esp, stack_top; Point esp to top of our stack

	;      Call our kernel's main function
	extern kernel_main; Declare that kernel_main is defined elsewhere
	call   kernel_main; Transfer control to our Rust code

	;   Disable interrupts and halt forever if we ever return
	cli ; Disable interrupts

.hang:
	hlt ; Halt the CPU
	jmp .hang; If we wake up (e.g., from NMI), halt again

.end:
