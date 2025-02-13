# Global Descriptor Table (GDT)

## What is it

The GDT serves as a fundamental data structure in x86 architecture, playing a crucial role in memory management and protection. When our computer starts, it begins in `real mode`, a simple operating mode that provides direct access to memory and I/O devices. However we need to switch to `protected mode`, which introduces memory protection, virtual memory, and privilege levels.

Think of `protected mode` as establishing different security clearance levels in a building. The GDT acts like the security system that defines who can access what. While my earlier comparison to `sudo` captured the basic idea of privilege levels, the reality is more sophisticated. Instead of just "admin" and "user", the x86 architecture provides four rings (0-3), where ring 0 is the most privileged (kernel space) and ring 3 is the least privileged (user space). Each ring has specific permissions and restrictions, all defined in our GDT.

The GDT is essential not just for security, but also for the basic operation of `protected mode`. Without a properly configured GDT, the CPU cannot execute `protected mode` code at all.

For more information go to [OSDev](https://wiki.osdev.org/Global_Descriptor_Table)

## My Technical Approach

My approach was as follows: I would start at the `boot.asm` & setup the multiboot. This will then call `gdt_init`, which is a Rust function. `gdt_init` will setup the `GDT_Entries` & ensure that it creates the correct struct pointer that will be passed to `gdt.asm`. `gdt.asm` will place the entries in the correct registers.

The multiboot setup is crucial because it ensures our kernel is loaded correctly by the bootloader and meets the Multiboot Specification, which is a standardized way for bootloaders to load operating systems.

Here are some snippets to give you a better idea:

```nasm
; Both Rust functions
extern gdt_init
extern kernel_main

_start:
    ; The bootloader has loaded us into 32-bit protected mode
    ; but we need to set up our own GDT for proper segmentation
    call   gdt_init
    call   kernel_main
```

```rust
#[no_mangle] // Ensure rustc doesn't mangle the symbol name for external linking
pub fn gdt_init() {
    // Create the GDT descriptor structure
    // size is (total_size - 1) because the limit field is maximum addressable unit
    let gdt_descriptor = GDTDescriptor { 
        size: (size_of::<GdtGates>() - 1) as u16,  // Size must be one less than actual size
        offset: 0x00000800,  // Place GDT at specified address in memory
    }; 
    // Call assembly function to load GDT register (GDTR)
    gdt_flush(&gdt_descriptor as *const _);
}
```

Here's how our GDT entries are structured:

```rust
// Each GDT entry is 64 bits (8 bytes)
pub struct Gate(pub u64);  

#[no_mangle]
#[link_section = ".gdt"]  // Place in special GDT section for linking
pub static GDT_ENTRIES: GdtGates = [
    // Null descriptor - Required by CPU specification for error checking
    Gate(0),
    // Kernel Code Segment: Ring 0, executable, non-conforming
    // Parameters: base=0, limit=max, access=0b10011010 (present, ring 0, code), flags=0b1100 (32-bit, 4KB granularity)
    Gate::new(0, !0, 0b10011010, 0b1100),  
    // Kernel Data Segment: Ring 0, writable, grow-up
    // Parameters: base=0, limit=max, access=0b10010010 (present, ring 0, data), flags=0b1100 (32-bit, 4KB granularity)
    Gate::new(0, !0, 0b10010010, 0b1100),  
    // User Code Segment: Ring 3, executable, non-conforming
    // Parameters: base=0, limit=max, access=0b11111010 (present, ring 3, code), flags=0b1100 (32-bit, 4KB granularity)
    Gate::new(0, !0, 0b11111010, 0b1100),  
    // User Data Segment: Ring 3, writable, grow-up
    // Parameters: base=0, limit=max, access=0b11110010 (present, ring 3, data), flags=0b1100 (32-bit, 4KB granularity)
    Gate::new(0, !0, 0b11110010, 0b1100),  
];
```

The assembly code in `gdt.asm` that actually loads the GDT:

```nasm
gdt_flush:
    ; Load GDT descriptor structure address from stack
    mov  eax, [esp + 4]
    lgdt [eax]

    ; Enable protected mode by setting the first bit of CR0
    mov eax, cr0
    or  eax, 1
    mov cr0, eax

    ; Set up segment registers with appropriate selectors
    ; 0x10 points to the kernel data segment (third GDT entry)
    mov eax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Far jump to flush pipeline and load CS with kernel code selector (0x08)
    ; This is necessary to fully enter protected mode
    jmp 0x08:.flush

.flush:
    ret
```

I am not going in-depth on why specific things happen, but each step is crucial for properly initializing protected mode.

## Considerations

I considered using inline assembly, but the reason I did not in the end was because it had a few known bugs in Rust. I felt much more comfortable using actual `asm` files, instead of doing it inline. Inline assembly in Rust can be particularly problematic when dealing with low-level CPU features, as the compiler's assumptions about register usage and calling conventions might conflict with what we need for GDT setup.

### Use C or Assembly instead of Rust

Since C is a much older & better documented program than Rust, I considered using C for the `gdt_init`, instead of Rust. I did not in the end, because I wanted to stay true to my "pure" Rust kernel & I felt like it would complicate things much more. Using C would have required additional complexity in the build system to handle multiple languages and their interaction.

## Linker

Force the Linker to put GDT before the rest of the code. You can do it without this, but you will have issues if you would like to place it in a specific address. If that is not necessary for you, you can ignore this. The specific placement of the GDT in memory can be important for some system designs, particularly when dealing with memory management and virtual memory setup. This was my approach:

```
  /* Start at 2MB */
  . = 2M;


  .gdt 0x800 : ALIGN(0x800)
    {
    gdt_start = .;
    *(.gdt)
    gdt_end = .;
  }
```

### Memory Layout Considerations

When placing the GDT at a specific address, it's important to ensure that:

1. The address is accessible during the transition to protected mode
2. The address doesn't conflict with other important system structures
3. The address is properly aligned for optimal performance
