use super::{
	allocator::EARLY_PHYSICAL_ALLOCATOR, get_kernel_physical_end,
	get_kernel_physical_start, MemorySegment, PhysAddr, KERNEL_OFFSET,
	PAGE_SIZE,
};
use crate::{log_warn, sync::Mutex};
use core::{
	cell::OnceCell,
	sync::atomic::{AtomicUsize, Ordering},
	usize,
};

const TOTAL_FRAMES: usize = usize::MAX / PAGE_SIZE + 1;
const BITMAP_ENTRY_SIZE_BITS: usize = u64::BITS as usize;
const BITMAP_ARRAY_SIZE: usize =
	(TOTAL_FRAMES + BITMAP_ENTRY_SIZE_BITS - 1) / BITMAP_ENTRY_SIZE_BITS;

pub static FRAME_ALLOCATOR: Mutex<OnceCell<FrameAllocator>> =
	Mutex::new(OnceCell::new());

static FRAME_BITMAP: Mutex<[u64; BITMAP_ARRAY_SIZE]> =
	Mutex::new([u64::MAX; BITMAP_ARRAY_SIZE]);

pub struct FrameAllocator {
	next_free_idx: AtomicUsize,
}

impl FrameAllocator {
	pub const fn new() -> Self {
		Self {
			next_free_idx: AtomicUsize::new(0),
		}
	}

	/// Initializes the static frame bitmap based on the memory map.
	/// Marks known used areas like the kernel and the bitmap itself.
	/// MUST be called only once during kernel initialization.
	pub fn init(&self) {
		let mut bitmap = FRAME_BITMAP.lock();
		let guard = EARLY_PHYSICAL_ALLOCATOR.lock();
		let regions = guard
			.get()
			.expect("Memblock has not been initialized")
			.mem_region();

		for region in regions.iter() {
			let start_addr = region.base();
			let end_addr = start_addr + region.size();

			let first_frame_idx =
				(start_addr.as_usize() + PAGE_SIZE - 1) / PAGE_SIZE;
			let last_frame_idx = end_addr.as_usize() / PAGE_SIZE;

			for frame_idx in first_frame_idx..last_frame_idx {
				if frame_idx < TOTAL_FRAMES {
					let entry_idx = frame_idx / BITMAP_ENTRY_SIZE_BITS;
					let bit_idx = frame_idx % BITMAP_ENTRY_SIZE_BITS;
					bitmap[entry_idx] &= !(1 << bit_idx);
				}
			}
		}

		let kernel_start_frame =
			get_kernel_physical_start().as_usize() / PAGE_SIZE;
		let kernel_end_frame =
			(get_kernel_physical_end().as_usize() + PAGE_SIZE - 1) / PAGE_SIZE;
		self.mark_range_used(&mut bitmap, kernel_start_frame, kernel_end_frame);

		let bitmap_virt_addr = bitmap.as_ptr() as usize;
		let bitmap_phys_addr = bitmap_virt_addr
			.checked_sub(KERNEL_OFFSET)
			.expect("Failed to calculate bitmap physical address");
		let bitmap_size_bytes = BITMAP_ARRAY_SIZE * size_of::<u64>();

		let bitmap_start_frame = bitmap_phys_addr / PAGE_SIZE;
		let bitmap_end_frame =
			(bitmap_phys_addr + bitmap_size_bytes + PAGE_SIZE - 1) / PAGE_SIZE;
		self.mark_range_used(&mut bitmap, bitmap_start_frame, bitmap_end_frame);
	}

	/// Allocates a single physical frame.
	pub fn allocate_frame(&self) -> Option<PhysAddr> {
		let mut bitmap = FRAME_BITMAP.lock();
		let start_idx = self.next_free_idx.load(Ordering::Relaxed);

		for entry_idx in start_idx..BITMAP_ARRAY_SIZE {
			if bitmap[entry_idx] != u64::MAX {
				for bit_idx in 0..BITMAP_ENTRY_SIZE_BITS {
					let mask = 1 << bit_idx;
					if (bitmap[entry_idx] & mask) == 0 {
						let frame_idx =
							entry_idx * BITMAP_ENTRY_SIZE_BITS + bit_idx;

						if frame_idx >= TOTAL_FRAMES {
							continue;
						}

						bitmap[entry_idx] |= mask;

						self.next_free_idx.store(entry_idx, Ordering::Relaxed);

						return Some(PhysAddr::new(frame_idx * PAGE_SIZE));
					}
				}
			}
		}

		None
	}

	/// Deallocates a single physical frame.
	pub fn deallocate_frame(&self, frame: PhysAddr) {
		let frame_idx = frame.as_usize() / PAGE_SIZE;
		if frame_idx >= TOTAL_FRAMES {
			log_warn!(
				"Attempted to deallocate frame outside tracked range: {:?}",
				frame
			);
			return;
		}

		let entry_idx = frame_idx / BITMAP_ENTRY_SIZE_BITS;
		let bit_idx = frame_idx % BITMAP_ENTRY_SIZE_BITS;
		let mask = 1 << bit_idx;

		let mut bitmap = FRAME_BITMAP.lock();

		if (bitmap[entry_idx] & mask) == 0 {
			log_warn!("Double free detected for frame: {:?}", frame);
			return;
		}

		bitmap[entry_idx] &= !mask;

		if entry_idx < self.next_free_idx.load(Ordering::Relaxed) {
			self.next_free_idx.store(entry_idx, Ordering::Relaxed);
		}
	}

	// Helper to mark a range as used (sets bits)
	fn mark_range_used(
		&self,
		bitmap: &mut [u64; BITMAP_ARRAY_SIZE],
		start_frame: usize,
		end_frame: usize,
	) {
		for frame_idx in start_frame..end_frame {
			if frame_idx < TOTAL_FRAMES {
				let entry_idx = frame_idx / BITMAP_ENTRY_SIZE_BITS;
				let bit_idx = frame_idx % BITMAP_ENTRY_SIZE_BITS;
				bitmap[entry_idx] |= 1 << bit_idx;
			}
		}
	}
}
