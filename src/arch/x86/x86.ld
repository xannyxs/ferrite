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

SECTIONS
  {
  /* Start at 2MB */
  . = 2M;

  gdt_start = .;
  .gdt 0x800 : ALIGN(0x800)
    {
    *(.gdt)
  }
  gdt_end = .;

  /* Text section with multiboot header */
  multiboot_start = .;
  .text BLOCK(4K) : ALIGN(4K)
    {
    KEEP(*(.multiboot))
    *(.text)
  }
  multiboot_end = .;

  /* Read-only data. */
  .rodata BLOCK(4K) : ALIGN(4K)
    {
    *(.rodata)
  }

  /* Read-write data (initialized) */
  .data BLOCK(4K) : ALIGN(4K)
    {
    *(.data)
  }

  /* Read-write data (uninitialized) and stack */
  .bss BLOCK(4K) : ALIGN(4K)
    {
    *(COMMON)
    *(.bss)
  }
}
