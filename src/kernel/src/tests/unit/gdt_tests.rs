use crate::println;
use core::{
	arch::asm,
	ptr::{read_volatile, write_volatile},
};

#[test_case]
fn test_low_memory_access() {
	unsafe {
		let ptr: *mut u8 = core::ptr::with_exposed_provenance_mut(0x1000);

		ptr.write_volatile(0x55);
		let value = ptr.read_volatile();

		assert_eq!(value, 0x55);
	}
}

#[test_case]
fn test_video_memory_access() {
	unsafe {
		let ptr: *mut u8 = core::ptr::with_exposed_provenance_mut(0xb8000);
		ptr.write_volatile(b'X');
		let value = ptr.read_volatile();

		assert_eq!(value, b'X');
	}
}

#[test_case]
fn test_safe_memory_region() {
	unsafe {
		let ptr: *mut u8 = core::ptr::with_exposed_provenance_mut(0x100000);
		ptr.write_volatile(0xaa);
		let value = ptr.read_volatile();
		assert_eq!(value, 0xaa);
	}
}
