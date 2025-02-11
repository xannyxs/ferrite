use crate::{arch::x86::boot::GDT_ENTRIES, println};

pub fn print_gdt() {
	println!("Global Descriptor Table (GDT)");
	println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

	for (index, mut gate) in GDT_ENTRIES.iter().map(|g| *g).enumerate() {
		let descriptor_type = match index {
			0 => "Null Descriptor",
			1 => "Kernel Code Segment",
			2 => "Kernel Data Segment",
			3 => "User Code Segment",
			4 => "User Data Segment",
			_ => "Unknown Segment",
		};

		let ring_level = match index {
			0 => "N/A",
			1 | 2 => "Ring 0 (Kernel)",
			3 | 4 => "Ring 3 (User)",
			_ => "Unknown",
		};

		println!(
			"Entry [{}]:\n \
     Type: {}\n \
     Privilege: {}\n \
     Access: {:#010b}\n \
     Flags: {:#06b}\n",
			index,
			descriptor_type,
			ring_level,
			gate.access(),
			gate.flags()
		);
		println!();
	}
}
