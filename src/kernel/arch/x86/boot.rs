use crate::arch::x86::gdt;

pub type GdtGates = [gdt::Gate; 5];

#[no_mangle]
//#[link_section = ".boot.data"]
pub static GDT_ENTRIES: GdtGates = [
	gdt::Gate(0), // Null Descriptor
	#[cfg(target_arch = "x86")]
	gdt::Gate::new(0, !0, 0b10011010, 0b1100), // Kernel Mode Code Segment (32bit)
	gdt::Gate::new(0, !0, 0b10010010, 0b1100), // Kernel Mode Data Segment
	gdt::Gate::new(0, !0, 0b11111010, 0b1100), // User Mode Code Segment
	gdt::Gate::new(0, !0, 0b11110010, 0b1100), // User Mode Data Segment
];

//gdt::Gate(0), // TSS 1
//gdt::Gate(0), // TSS 2
