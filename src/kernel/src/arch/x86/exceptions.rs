use super::idt::InterruptDescriptorEntry as Entry;

#[doc(hidden)]
#[derive(Clone)]
#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
	pub divide_by_zero: Entry,
	pub debug: Entry,
	pub non_maskable_interrupt: Entry,
	pub breakpoint: Entry,
	pub overflow: Entry,
	pub bound_range_exceeded: Entry,
	pub invalid_opcode: Entry,
	pub device_not_available: Entry,
	pub double_fault: Entry,
	pub invalid_tss: Entry,
	pub segment_not_present: Entry,
	pub stack_segment_fault: Entry,
	pub general_protection_fault: Entry,
	pub page_fault: Entry,
	pub x87_floating_point: Entry,
	pub alignment_check: Entry,
	pub machine_check: Entry,
	pub simd_floating_point: Entry,
	pub virtualization: Entry,
	pub security_exception: Entry,
}
