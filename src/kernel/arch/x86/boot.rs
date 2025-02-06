use crate::arch::x86::gdt;

// We need to store our GDT entries somewhere
static mut GDT_ENTRIES: [gdt::Gate; 3] = [
	gdt::Gate(0), // Null Descriptor
	gdt::Gate(0), // Code Segment Descriptor
	gdt::Gate(0), // Data Segment Descriptor
];

#[no_mangle]
#[allow(static_mut_refs)]
pub fn gdt_init() -> *const gdt::Gate {
	unsafe {
		GDT_ENTRIES[0] = gdt::Gate(0); // Null Descriptor
		GDT_ENTRIES[1] = gdt::Gate::new(0, !0, 0b10011010, 0b11001111); // Code Segment
		GDT_ENTRIES[2] = gdt::Gate::new(0, !0, 0b10010010, 0b11001111); // Data Segment

		// Return pointer to our GDT
		GDT_ENTRIES.as_ptr()
	}
}
