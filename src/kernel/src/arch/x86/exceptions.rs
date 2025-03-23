use super::cpu::reboot;
use crate::{arch::x86::cpu::halt, println, println_serial};

pub type InterruptHandler = extern "x86-interrupt" fn(InterruptFrame);
pub type InterruptHandlerWithError =
	extern "x86-interrupt" fn(frame: InterruptFrame, error_code: u32);

/// CPU-pushed interrupt stack frame in 32-bit mode
#[repr(C)]
#[derive(Debug)]
pub struct InterruptFrame {
	pub instruction_pointer: u32,
	pub code_segment: u32,
	pub eflags: u32,
	pub stack_pointer: u32,
	pub stack_segment: u32,
}

#[derive(Copy, Clone)]
pub enum InterruptHandlerType {
	Regular(InterruptHandler),
	WithErrorCode(InterruptHandlerWithError),
}

pub static INTERRUPT_HANDLERS: [InterruptHandlerType; 21] = [
	InterruptHandlerType::Regular(divide_by_zero_handler),
	InterruptHandlerType::Regular(debug_interrupt_handler),
	InterruptHandlerType::Regular(non_maskable_interrupt_handler),
	InterruptHandlerType::Regular(breakpoint_handler),
	InterruptHandlerType::Regular(overflow_handler),
	InterruptHandlerType::Regular(bound_range_exceeded_handler),
	InterruptHandlerType::Regular(invalid_opcode),
	InterruptHandlerType::Regular(device_not_available),
	InterruptHandlerType::WithErrorCode(double_fault),
	InterruptHandlerType::Regular(coprocessor_segment_overrun),
	InterruptHandlerType::WithErrorCode(invalid_tss),
	InterruptHandlerType::WithErrorCode(segment_not_present),
	InterruptHandlerType::WithErrorCode(stack_segment_fault),
	InterruptHandlerType::WithErrorCode(general_protection_fault),
	InterruptHandlerType::WithErrorCode(page_fault),
	InterruptHandlerType::Regular(x87_floating_point),
	InterruptHandlerType::WithErrorCode(alignment_check),
	InterruptHandlerType::Regular(machine_check),
	InterruptHandlerType::Regular(simd_floating_point),
	InterruptHandlerType::Regular(virtualization),
	InterruptHandlerType::WithErrorCode(security_exception),
];

pub extern "x86-interrupt" fn divide_by_zero_handler(frame: InterruptFrame) {
	println!("EXCEPTION: DIVIDE BY ZERO (#DE)");
	println!("===============================");

	println!("Instruction Pointer: 0x{:08x}", frame.instruction_pointer);
	println!("Code Segment: 0x{:04x}", frame.code_segment);
	println!("EFLAGS: 0x{:08x}", frame.eflags);
	println!("Stack Pointer: 0x{:08x}", frame.stack_pointer);
	println!("Stack Segment: 0x{:04x}", frame.stack_segment);

	if frame.code_segment & 0x3 == 0 {
		println!("CRITICAL: Divide by zero in kernel code!");
		panic!("KERNEL PANIC: Cannot divide by zero in kernel mode");
	}

	println!("User process attempted division by zero");
	println!("Terminating process...");

	halt();
}

pub extern "x86-interrupt" fn debug_interrupt_handler(frame: InterruptFrame) {
	println!("EXCEPTION: DEBUG EXCEPTION (#DB)");
	println!("===============================");
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(
	frame: InterruptFrame,
) {
	println!("Non-maskable interrupt (NMI)");
}

pub extern "x86-interrupt" fn breakpoint_handler(frame: InterruptFrame) {
	println!("Breakpoint exception (#BP)");
}

pub extern "x86-interrupt" fn overflow_handler(frame: InterruptFrame) {
	println!("Overflow exception (#OF)");
}

pub extern "x86-interrupt" fn bound_range_exceeded_handler(
	frame: InterruptFrame,
) {
	println!("BOUND range exceeded exception (#BR)");
}

pub extern "x86-interrupt" fn invalid_opcode(frame: InterruptFrame) {
	println!("Invalid opcode exception (#UD)");
}

pub extern "x86-interrupt" fn device_not_available(frame: InterruptFrame) {
	println!("Device not available exception (#NM)");
}

pub extern "x86-interrupt" fn double_fault(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("Double fault exception (#DF)");

	reboot();
}

pub extern "x86-interrupt" fn coprocessor_segment_overrun(
	frame: InterruptFrame,
) {
	println!("Coprocessor segment overrun");
}

pub extern "x86-interrupt" fn invalid_tss(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("Invalid TSS exception (#TS)");
}

pub extern "x86-interrupt" fn segment_not_present(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("Segment not present exception (#NP)");
}

pub extern "x86-interrupt" fn stack_segment_fault(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("Stack-segment fault (#SS)");
}

pub extern "x86-interrupt" fn general_protection_fault(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("EXCEPTION: GENERAL PROTECTION FAULT (#GP)");
	println!("===============================");

	println!("Error Code: 0x{:04x}", error_code);
	println!("Debug information: {:?}", frame);
	println_serial!("Debug information: {:?}", frame);

	halt();
}

pub extern "x86-interrupt" fn page_fault(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("EXCEPTION: PAGE FAULT EXCEPTION (#PF)");
	println!("===============================");

	println!("Error Code: 0x{:04x}", error_code);
	println!("Debug information: {:?}", frame);
	println_serial!("Debug information: {:?}", frame);

	halt();
}

pub extern "x86-interrupt" fn x87_floating_point(frame: InterruptFrame) {
	println!("x87 floating-point exception (#MF)");
}

pub extern "x86-interrupt" fn alignment_check(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("Alignment check exception (#AC)");
}

pub extern "x86-interrupt" fn machine_check(frame: InterruptFrame) {
	println!("Machine check exception (#MC)");
}

pub extern "x86-interrupt" fn simd_floating_point(frame: InterruptFrame) {
	println!("SIMD floating-point exception (#XM)");
}

pub extern "x86-interrupt" fn virtualization(frame: InterruptFrame) {
	println!("Virtualization exception (#VE)");
}

pub extern "x86-interrupt" fn security_exception(
	frame: InterruptFrame,
	error_code: u32,
) {
	println!("Security exception (#SX)");
}
