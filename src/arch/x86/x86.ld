ENTRY(_start)
/*OUTPUT_FORMAT(elf32-i386)*/

SECTIONS
{
    /* Start at 1MB as before */
    . = 0x00100000;

    /* Text section with multiboot header */
    .text : ALIGN(CONSTANT(MAXPAGESIZE)) {
        . = ALIGN(8);
        KEEP(*(.multiboot))
        _TEXT_START_ = .;
        *(.text*)           /* Changed to .text* to catch all text sections */
        *(.rodata*)         /* Added for Rust's read-only data */
        *(.eh_frame)        /* Added for exception handling information */
        *(.eh_frame_hdr)    /* Added for exception frame headers */
        _TEXT_END_ = .;
    }

    /* Data section */
    .data : ALIGN(CONSTANT(MAXPAGESIZE)) {
        _DATA_START_ = .;
        *(.data*)           /* Changed to .data* to catch all data sections */
        *(.got*)            /* Added for global offset table */
        *(.got.plt*)        /* Added for procedure linkage table */
        _DATA_END_ = .;
    }

    /* BSS section */
    .bss : ALIGN(CONSTANT(MAXPAGESIZE)) {
        _BSS_START_ = .;
        *(.bss*)            /* Changed to .bss* to catch all bss sections */
        *(COMMON)           /* Added for common symbols */
        _BSS_END_ = .;
    }
}
