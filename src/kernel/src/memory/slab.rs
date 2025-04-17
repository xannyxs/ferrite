//! Implements a slab allocator for fixed-size memory allocations.

use super::{VirtAddr, PAGE_SIZE};
use crate::{
	collections::intrusive_linked_list::{IntrusiveLinkedList, IntrusiveNode},
	log_error,
	memory::allocator::BUDDY_PAGE_ALLOCATOR,
	println_serial,
	sync::Locked,
};
use core::{
	alloc::{GlobalAlloc, Layout},
	mem,
	ops::Add,
	ptr::NonNull,
};

#[derive(Debug)]
struct Slab {
	list: IntrusiveNode<Slab>,
	cache: *const SlabCache,
	base_vaddr: VirtAddr,
	objects_in_use: usize,
	first_free_object: Option<NonNull<u8>>,
}

/// Represents a single slab of memory containing multiple fixed-size objects.
/// This struct itself resides at the beginning of the allocated slab memory.
pub struct SlabCache {
	slabs_full: IntrusiveLinkedList<Slab>,
	slabs_partial: IntrusiveLinkedList<Slab>,
	slabs_free: IntrusiveLinkedList<Slab>,

	object_size: usize,
	slab_order: usize,
	objects_per_slab: usize,
	// name: &'static str,
	// lock: Spinlock
}

unsafe impl Send for SlabCache {}
unsafe impl Sync for SlabCache {}

unsafe impl GlobalAlloc for Locked<SlabCache> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe { self.lock().alloc(layout) }
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe { self.lock().dealloc(ptr, layout) }
	}
}

// Allocations
impl SlabCache {
	/// Allocates one object from this slab cache.
	///
	/// Attempts to reuse an object from a partially full or free slab.
	/// If none are available, allocates a new slab from the underlying
	/// Buddy Allocator and returns the first object.
	///
	/// # Safety
	/// The caller receives a raw pointer to uninitialized memory. The layout
	/// size must be appropriate for this cache (<= `self.object_size`). This
	/// function assumes exclusive mutable access (`&mut self`).
	#[allow(clippy::expect_used)]
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		use core::ptr;

		assert!(layout.size() <= self.object_size);

		if !self.slabs_partial.is_empty() {
			match self.slabs_partial.pop_front() {
				Some(node) => {
					return self.add_object(node).unwrap_or(ptr::null_mut());
				}
				None => return ptr::null_mut(),
			};
		}

		if !self.slabs_free.is_empty() {
			match self.slabs_free.pop_front() {
				Some(node) => {
					return self.add_object(node).unwrap_or(ptr::null_mut());
				}
				None => return ptr::null_mut(),
			};
		}

		let ptr: *mut u8 = {
			let mut buddy = BUDDY_PAGE_ALLOCATOR.lock();

			match buddy.get_mut() {
				Some(buddy) => {
					let size_to_alloc = (1 << self.slab_order) * PAGE_SIZE;
					let layout =
						Layout::from_size_align(size_to_alloc, PAGE_SIZE)
							.expect("Failed to create Buddy Layout");

					unsafe { buddy.alloc(layout) }
				}
				None => return ptr::null_mut(),
			}
		};

		if ptr.is_null() {
			println_serial!(
				"Buddy allocator failed to provide memory for new slab!"
			);
			return ptr::null_mut();
		}

		let addr: VirtAddr = (ptr as usize).into();
		let slab_ptr = addr.as_mut_ptr::<Slab>();
		let slab_size = (1 << self.slab_order) * PAGE_SIZE;

		let object_start =
			(addr + size_of::<Slab>()).align_up(align_of::<usize>());
		let object_end = addr + slab_size;
		let object_area_size = object_end.as_usize() - object_start.as_usize();

		let objects_in_slab = object_area_size / self.object_size;
		let object_to_return_ptr = self
			.setup_free_list(object_start, objects_in_slab)
			.expect("Newly initialized slab has no free objects!")
			.as_ptr();

		let mut next_free_obj_option = None;
		if objects_in_slab > 1 {
			let next_free_raw =
				unsafe { *(object_to_return_ptr as *const *mut u8) };
			next_free_obj_option = NonNull::new(next_free_raw);
		}

		unsafe {
			ptr::write(
				slab_ptr,
				Slab {
					list: IntrusiveNode::new(NonNull::new(slab_ptr)),
					cache: self as *const Self,
					base_vaddr: object_start,
					objects_in_use: 1,
					first_free_object: next_free_obj_option,
				},
			);
		}

		let node_ptr = unsafe { ptr::addr_of_mut!((*slab_ptr).list) };

		println_serial!(
			"Added new slab {:p} node {:p} to partial list",
			slab_ptr,
			node_ptr
		);

		self.slabs_partial.push_back(NonNull::new(node_ptr));

		object_to_return_ptr
	}

	/// Deallocates an object previously allocated from this slab cache.
	///
	/// Finds the slab containing the `ptr`, adds the object back to the slab's
	/// internal free list, decrements the usage count, and moves the slab
	/// between the `slabs_full`, `slabs_partial`, and `slabs_free` lists
	/// as appropriate.
	///
	/// # Safety
	/// The caller *must* ensure that `ptr` points to a valid object previously
	/// allocated from *this specific `SlabCache` instance* with a compatible
	/// `layout`. Double-freeing or freeing a pointer not belonging to this
	/// cache leads to undefined behavior. Assumes exclusive mutable access
	/// (`&mut self`).
	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		use core::ptr;

		debug_assert!(
			layout.size() <= self.object_size,
			"Layout size mismatch"
		);
		debug_assert!(!ptr.is_null(), "Attempted to deallocate null pointer");
		debug_assert!(
			self.object_size >= mem::size_of::<usize>(),
			"Object size too small for free list link"
		);

		let addr = ptr as usize;
		let slab_alloc_size = (1 << self.slab_order) * PAGE_SIZE;

		let mask = !(slab_alloc_size - 1);
		let slab_ptr: *mut Slab = ptr::with_exposed_provenance_mut(addr & mask);

		match unsafe { slab_ptr.as_mut() } {
			Some(slab) => {
				let next_free_ptr_val = match slab.first_free_object {
					Some(head) => head.as_ptr() as usize,
					None => 0,
				};

				unsafe { (ptr as *mut usize).write(next_free_ptr_val) };

				slab.first_free_object = NonNull::new(ptr);

				let node_ptr = NonNull::new(ptr::addr_of_mut!(slab.list));
				if slab.objects_in_use == self.objects_per_slab {
					self.slabs_full.remove(node_ptr);
					self.slabs_partial.push_back(node_ptr);
				} else if slab.objects_in_use == 1 {
					self.slabs_partial.remove(node_ptr);
					self.slabs_free.push_back(node_ptr);
				}

				slab.objects_in_use -= 1;
			}
			None => {
				log_error!(
					"Could not get slab ref at {:p} for object {:p}",
					slab_ptr,
					ptr
				);
			}
		}
	}
}

// Public Interface
impl SlabCache {
	/// Creates a new `SlabCache` for objects of `size` bytes.
	///
	/// Calculates the number of objects fitting in a slab based on the
	/// `slab_order` (which determines the total slab size = `PAGE_SIZE` <<
	/// `slab_order`).
	///
	/// # Panics
	/// Panics if the calculated slab size is too small to hold even one object
	/// plus the required `Slab` metadata.
	pub fn new(size: usize, slab_order: usize) -> Self {
		let object_align = mem::align_of::<usize>();
		let metadata_size = mem::size_of::<Slab>();
		let slab_size = PAGE_SIZE << slab_order;

		let offset = (metadata_size + object_align - 1) & !(object_align - 1);
		let usable_space = slab_size - offset;

		let mut objects_per_slab = 0;
		if size > 0 {
			objects_per_slab = usable_space / size;
		};

		if objects_per_slab == 0 && size > 0 {
			panic!("Slab order {} is too small for object size {} with on-slab metadata!", slab_order, size);
		}

		Self {
			slabs_full: IntrusiveLinkedList::new(),
			slabs_partial: IntrusiveLinkedList::new(),
			slabs_free: IntrusiveLinkedList::new(),
			object_size: size,
			slab_order,
			objects_per_slab,
		}
	}
}

// Private interface
impl SlabCache {
	fn setup_free_list(
		&self,
		start: VirtAddr,
		count: usize,
	) -> Option<NonNull<u8>> {
		use core::ptr;

		if count == 0 || self.object_size < mem::size_of::<*mut u8>() {
			return None;
		}

		let mut current_ptr = start.as_mut_ptr::<u8>();
		for i in 0..(count - 1) {
			let next_ptr_val = start.add((i + 1) * self.object_size);
			unsafe {
				ptr::write(current_ptr as *mut usize, next_ptr_val.as_usize())
			};
			current_ptr =
				start.add((i + 1) * self.object_size).as_mut_ptr::<u8>();
		}

		unsafe { ptr::write(current_ptr as *mut usize, 0) };
		NonNull::new(start.as_mut_ptr::<u8>())
	}

	fn add_object(
		&mut self,
		mut popped_node: NonNull<IntrusiveNode<Slab>>,
	) -> Option<*mut u8> {
		let slab = unsafe { popped_node.as_mut().container_mut()? };

		let object_ptr = slab.first_free_object.take()?.as_ptr();

		slab.first_free_object = unsafe {
			let next_free_raw = *(object_ptr as *const *mut u8);
			NonNull::new(next_free_raw)
		};

		slab.objects_in_use += 1;

		if slab.objects_in_use == self.objects_per_slab {
			println_serial!("Slab {:p} became full", popped_node.as_ptr());
			self.slabs_full.push_front(Some(popped_node));
		} else {
			self.slabs_partial.push_front(Some(popped_node));
		}

		Some(object_ptr)
	}
}
