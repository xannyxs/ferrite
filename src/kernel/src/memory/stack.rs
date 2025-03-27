use super::MemorySegment;
use crate::sync::locked::Locked;
use core::{
	cell::OnceCell,
	fmt,
	sync::atomic::{AtomicBool, Ordering},
};
use kernel_sync::Mutex;

extern "C" {
	static stack_bottom: u8;
}

/// Global, thread-safe container (`Mutex<OnceCell>`) for the kernel's
/// `KernelStack` info. Ensures safe concurrent access and one-time
/// initialization during kernel setup.
pub static STACK: Mutex<OnceCell<KernelStack>> = Mutex::new(OnceCell::new());

/// Manages the kernel's stack memory region
pub struct KernelStack {
	bottom: usize,
	size: usize,
}

impl fmt::Debug for KernelStack {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		return f
			.debug_struct("Kernel Stack")
			.field("Bottom", &format_args!("0x{:x}", self.bottom))
			.field("Size", &self.size)
			.finish();
	}
}

impl KernelStack {
	/// Creates a new KernelStack if not already initialized.
	/// Returns Err if already initialized.
	///
	/// # Safety
	/// - Must only be called during kernel initialization
	/// - The stack_bottom symbol must be properly defined in assembly
	/// - Assumes 16KB of stack space is available from stack_bottom
	pub const unsafe fn new() -> Self {
		return Self {
			bottom: 0,
			size: 0,
		};
	}

	pub unsafe fn init(&mut self) {
		let bottom = unsafe { &stack_bottom as *const u8 as usize };

		self.bottom = bottom;
		self.size = 16384;
	}

	/// Creates a mutable slice for memory segments, leaving buffer space.
	///
	/// # Safety
	/// - The returned slice points to raw stack memory
	/// - Caller must ensure no other references to this memory exist
	/// - The slice lifetime is 'static but must not outlive the stack
	/// - Buffer space (1024 bytes) must remain untouched
	pub unsafe fn create_segment_array(&self) -> &'static mut [MemorySegment] {
		let buffer = 1024;
		let available_size = self.size - buffer;
		let max_segments =
			available_size / core::mem::size_of::<MemorySegment>();
		let data = core::ptr::with_exposed_provenance_mut(self.bottom);

		unsafe {
			return core::slice::from_raw_parts_mut(data, max_segments);
		}
	}
}
