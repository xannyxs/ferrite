use super::{idt::InterruptDescriptorEntry as Entry, pic::send_eoi};
use crate::{println, println_serial};

/*
#[doc(hidden)]
#[derive(Clone)]
#[repr(C, align(16))]
pub struct Interrupts {
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
} */

type InterruptHandler = extern "x86-interrupt" fn();

pub static INTERRUPT_HANDLERS: [InterruptHandler; 15] = [
	divide_by_zero_handler,
	debug_interrupt_handler,
	non_maskable_interrupt_handler,
	breakpoint_handler,
	overflow_handler,
	bound_range_exceeded_handler,
	invalid_opcode,
	device_not_available,
	double_fault,
	invalid_tss,
	segment_not_present,
	stack_segment_fault,
	general_protection_fault,
	page_fault,
	x87_floating_point,
	/* alignment_check,
	machine_check,
	simd_floating_point,
	virtualization,
	security_exception, */
];

pub extern "x86-interrupt" fn divide_by_zero_handler() {
	println_serial!("Interrupt in divide_by_zero");
}

pub extern "x86-interrupt" fn debug_interrupt_handler() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn breakpoint_handler() {
	println_serial!("Interrupt in breakpoint_handler");
}

pub extern "x86-interrupt" fn overflow_handler() {
	println_serial!("Interrupt in overflow_handler");
}

pub extern "x86-interrupt" fn bound_range_exceeded_handler() {
	println_serial!("Interrupt in bound_range_exceeded_handler");
}

pub extern "x86-interrupt" fn invalid_opcode() {
	println_serial!("Interrupt in invalid_opcode");
}

pub extern "x86-interrupt" fn device_not_available() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn double_fault() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn invalid_tss() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn segment_not_present() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn stack_segment_fault() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn general_protection_fault() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn page_fault() {
	println_serial!("Interrupt in debug");
}

pub extern "x86-interrupt" fn x87_floating_point() {
	println_serial!("Interrupt in debug");
}
