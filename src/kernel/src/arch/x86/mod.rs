pub mod gdt;
pub mod idt;
pub mod multiboot;
pub mod pic;

/* -------------------------------------- */

// TODO: Look at file structure & add docs
#[doc(hidden)]
pub mod cpu;
#[doc(hidden)]
pub mod diagnostics;
#[doc(hidden)]
pub mod exceptions;
#[doc(hidden)]
pub mod io;

/* -------------------------------------- */

#[cfg(not(any(target_arch = "x86")))]
compile_error!("This code needs to be compiled for i686/x86!");

#[cfg(not(target_pointer_width = "32"))]
compile_error!("This code needs to be compiled for 32-bit architecture!");

/// Descriptor structure for CPU tables (IDT/GDT).
///
/// The location of descriptor tables is stored in their respective registers
/// (IDTR/GDTR) loaded using the LIDT/LGDT assembly instructions, which take
/// a pointer to this descriptor structure as an argument.
#[repr(C, packed)]
pub struct DescriptorTable {
	/// Size of the table in bytes, minus 1.
	size: u16,
	/// Linear address of the table (not physical address, paging applies).
	offset: u32,
}
