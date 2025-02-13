# KFS 02 - GDT & Stack

## Introduction

Let's proceed to the second `KFS` project. The first was doable and I felt confident doing the second one.

For this project, we had to implement a GDT (Global Descriptor Table). The GDT serves as a fundamental data structure in x86 architecture, playing a crucial role in memory management and protection. When our computer starts, it begins in `real mode`, a simple operating mode that provides direct access to memory and I/O devices. However we need to switch to protected mode, which introduces memory protection, virtual memory, and privilege levels.

Think of `protected mode` as establishing different security clearance levels in a building. The GDT acts like the security system that defines who can access what. While my earlier comparison to `sudo` captured the basic idea of privilege levels, the reality is more sophisticated. Instead of just "admin" and "user", the x86 architecture provides four rings (0-3), where ring 0 is the most privileged (kernel space) and ring 3 is the least privileged (user space). Each ring has specific permissions and restrictions, all defined in our GDT.

The GDT is essential not just for security, but also for the basic operation of `protected mode`. Without a properly configured GDT, the CPU cannot execute `protected mode` code at all.

## Goals

The project requires creating a GDT at 0x00000800 with entries for Kernel Data Space, Kernel Code Space, User Data Space, and User Code Space. Additionally, we need to add minimal PS/2 Keyboard Support and implement a basic shell with the commands `reboot` & `gdt`. The `gdt` command will print the GDT entries in a human-readable way.

## Technical Approach & Implementation

My journey began with studying the [OSDev](https://wiki.osdev.org/Global_Descriptor_Table) documentation. The concepts were initially overwhelming - terms like segment descriptors, privilege levels, and descriptor flags felt like learning a new language. After watching several YouTube tutorials ([here](https://www.youtube.com/watch?v=GvIJYELuaaE&t=5615s) & [here](https://www.youtube.com/watch?v=Wh5nPn2U_1w&t=429s)) about GDT implementation in Rust, things started to click.

I faced a choice: implement the GDT in Assembly or Rust. While Assembly would give more direct control, I chose Rust for its safety features and my growing familiarity with it. Here's how I structured the implementation:

The boot process begins in boot.asm, where we set up multiboot flags and prepare for the transition to protected mode. Then we call `gdt_init`, a Rust function that sets up our GDT:

```rust
#[no_mangle] // Ensure rustc doesn't mangle the symbol name for external linking
pub fn gdt_init() {
    // Create the GDT descriptor structure
    // size is (total_size - 1) because the limit field is maximum addressable unit
    let gdt_descriptor = GDTDescriptor { 
        size: (size_of::<GdtGates>() - 1) as u16,
        offset: 0x00000800,  // Place GDT at specified address
    }; 
    // Call assembly function to load GDT register (GDTR)
    gdt_flush(&gdt_descriptor as *const _);
}
```

Here's how our GDT entries are structured:

```rust
pub struct Gate(pub u64);  // Each GDT entry is 64 bits

#[no_mangle]
#[link_section = ".gdt"]  // Place in special GDT section for linking
pub static GDT_ENTRIES: GdtGates = [
    // Null descriptor - Required by CPU specification
    Gate(0),
    // Kernel Code Segment: Ring 0, executable, non-conforming
    Gate::new(0, !0, 0b10011010, 0b1100),  
    // Kernel Data Segment: Ring 0, writable, grow-up
    Gate::new(0, !0, 0b10010010, 0b1100),  
    // User Code Segment: Ring 3, executable, non-conforming
    Gate::new(0, !0, 0b11111010, 0b1100),  
    // User Data Segment: Ring 3, writable, grow-up
    Gate::new(0, !0, 0b11110010, 0b1100),  
];
```

Each Gate::new() call takes four parameters:

- base: The starting address of the segment (0 for flat memory model)
- limit: The maximum addressable unit (!0 means use entire address space)
- access: Defines segment privileges and type (explained in detail in the table below)
- flags: Controls granularity and size (0b1100 for 32-bit protected mode)

After setting up the GDT, I implemented basic keyboard support. While my current polling approach isn't ideal (it continuously checks for keystrokes), it works for our basic shell. A proper implementation would use interrupts to handle keyboard events, but that's a topic for future projects. The VGA driver from KFS_01 was adapted to create a simple shell interface, allowing for the `reboot` and `gdt` commands.

The system still experienced triple faults initially. The solution lay in the linker script - by using `#[link_section = ".gdt"]`, I ensured our GDT was placed at the correct memory address. The ordering is crucial: BIOS boot code, then GDT, then the rest of our kernel.

```
  /* Start at 2MB */
  . = 2M;


  .gdt 0x800 : ALIGN(0x800)
    {
    gdt_start = .;
    *(.gdt)
    gdt_end = .;
  }

  /* The rest... */
```

## Challenges

The challenges were mostly understanding the GDT. I struggled to grasp its purpose and exact workings. It took me reading several articles and watching multiple videos to finally understand what it's meant to do.

I also had no real experience with the linker. Finding the source of the triple fault was particularly frustrating, and it took quite a while before I realized the linker might not be placing the GDT at the correct address.

## Conclusion & Lesson Learned

I found that I needed to reread materials multiple times to fully grasp concepts. Fortunately, there was plenty of documentation available about the GDT and its implementation. Working with the GDT motivated me to document everything extensively, like these pages. I mainly do this to ensure I truly understand the functionality of each component I'm working with.
