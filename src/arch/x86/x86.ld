/* This linker script organizes the memory layout for an x86 kernel.
* It places the program at the 2MB mark in memory and arranges different
* types of data into separate 4KB-aligned sections:
* 
* - .text:   Contains executable code and multiboot header
* - .rodata: Holds read-only data like constants
* - .data:   Stores initialized variables
* - .bss:    Reserved for uninitialized variables
* 
* The 4KB alignment matches the x86 page size, enabling proper memory
* protection and efficient virtual memory management.
*/

ENTRY(_start)

SECTIONS {
  /* Start at 2MB */
  . = 2M;

  .gdt 0x800 : ALIGN(0x800) {
    *(.gdt)
  }

  .multiboot ALIGN(4K) : {
    KEEP(*(.multiboot))
  }

  .note.gnu.build-id ALIGN(4K) : {
    *(.note.gnu.build-id)
  }

  .boot ALIGN(4K) : {
    *(.boot)
  }

  . += 0xC0000000;

  .text ALIGN(4K) : AT(ADDR(.text) - 0xC0000000) {
    *(.text)
  }

  .rodata ALIGN(4K) : AT(ADDR(.rodata) - 0xC0000000) {
    *(.rodata)
  }

  .data ALIGN(4K) : AT(ADDR(.data) - 0xC0000000) {
    *(.data)
  }

  .bss ALIGN(4K) : AT(ADDR(.bss) - 0xC0000000) {
    *(COMMON)
    *(.bss)
  }

  _kernel_end = .;
}
