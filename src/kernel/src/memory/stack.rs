use super::MemorySegment;
use crate::sync::locked::Locked;
use core::{
	alloc::Layout,
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
	current_pos: usize,
}

pub struct StackAllocation {
	pub addr: *mut u8,
	pub size: usize,
	pub align: usize,
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
	pub unsafe fn new() -> Self {
		let bottom = unsafe { &stack_bottom as *const u8 as usize };

		return Self {
			bottom,
			size: 16384,
			current_pos: 0,
		};
	}

	pub fn size(&self) -> usize {
		return self.size;
	}

	pub unsafe fn allocate_from_stack(
		&mut self,
		layout: Layout,
	) -> Result<StackAllocation, &str> {
		use core::{ptr, slice};

		let aligned_pos =
			(self.current_pos + layout.align() - 1) & !(layout.align() - 1);
		let new_used_size = aligned_pos + layout.size();

		if new_used_size > self.size {
			return Err("Warning: Requested size exceeds stack space");
		}

		self.current_pos = new_used_size;

		let stack =
			ptr::with_exposed_provenance_mut(self.bottom + self.current_pos);

		return Ok(StackAllocation {
			addr: stack,
			size: layout.size(),
			align: layout.align(),
		});
	}

	pub unsafe fn create_usize_array(
		&mut self,
		map_size: usize,
	) -> Result<&'static mut [usize], &'static str> {
		use core::{
			ptr::with_exposed_provenance_mut, slice::from_raw_parts_mut,
		};

		let new_used_size =
			self.current_pos + map_size * core::mem::size_of::<usize>();

		if new_used_size > self.size {
			return Err("Warning: Requested size exceeds stack space");
		}

		self.current_pos = new_used_size;

		let stack = with_exposed_provenance_mut(self.bottom + self.current_pos);

		unsafe {
			return Ok(from_raw_parts_mut(stack, map_size));
		}
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
