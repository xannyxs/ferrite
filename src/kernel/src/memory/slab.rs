use super::{VirtAddr, PAGE_SIZE};
use crate::{
	collections::{
		intrusive_linked_list::{IntrusiveLinkedList, IntrusiveNode},
		linked_list::{LinkedList, Node},
	},
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

struct Slab {
	list: IntrusiveNode<Slab>,

	cache: *const SlabCache,
	base_vaddr: VirtAddr,
	objects_in_use: usize,
	first_free_object: Option<NonNull<u8>>,
}

pub struct SlabCache {
	slabs_full: IntrusiveLinkedList<Slab>,
	slabs_partial: IntrusiveLinkedList<Slab>,
	slabs_free: IntrusiveLinkedList<Slab>,

	object_size: usize,
	slab_order: usize,
	objects_per_slab: usize,
	// name: &'static str,
}

unsafe impl Send for SlabCache {}
unsafe impl Sync for SlabCache {}

unsafe impl GlobalAlloc for Locked<SlabCache> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut allocator = self.lock();

		unsafe {
			return allocator.alloc(layout);
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		let mut allocator = self.lock();

		unsafe { allocator.dealloc(ptr, layout) }
	}
}

// Allocations
impl SlabCache {
	// TODO:
	// - [] Check partial cachces
	// - [] check empty cachces
	// - [x] Ask Buddy Allocator for new memory
	pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
		use core::ptr;

		assert!(layout.size() <= self.object_size());

		if !self.slabs_partial.is_empty() {
			// TODO: Get addr for partial LL
			return ptr::null_mut();
		}

		if !self.slabs_free.is_empty() {
			// TODO: Get addr for empty LL
			return ptr::null_mut();
		}

		let ptr: *mut u8 = {
			let mut buddy = BUDDY_PAGE_ALLOCATOR.lock();

			match buddy.get_mut() {
				Some(buddy) => {
					let pages_to_alloc = 1 << self.slab_order;
					let size_to_alloc = pages_to_alloc * PAGE_SIZE;
					let buddy_layout =
						Layout::from_size_align(size_to_alloc, PAGE_SIZE)
							.expect("Failed to create Buddy Layout");

					unsafe { buddy.alloc(buddy_layout) }
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

		if self.object_size == 0 {
			println_serial!(
				"ERROR: New slab too small for objects size {} after metadata!",
				self.object_size
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
		let first_obj_ptr = self.setup_free_list(object_start, objects_in_slab);
		let object_to_return_nn =
			first_obj_ptr.expect("Newly initialized slab has no free objects!");
		let object_to_return_ptr = object_to_return_nn.as_ptr();

		let next_free_obj_option: Option<NonNull<u8>> = if objects_in_slab > 1 {
			unsafe {
				let next_free_raw = *(object_to_return_ptr as *const *mut u8);
				NonNull::new(next_free_raw)
			}
		} else {
			None
		};

		unsafe {
			ptr::write(
				slab_ptr,
				Slab {
					list: Default::default(),
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

		self.slabs_partial.push_front(node_ptr);

		return object_to_return_ptr;
	}

	pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
		let addr = ptr as usize;
	}
}

// Public Interface
impl SlabCache {
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

		return Self {
			slabs_full: IntrusiveLinkedList::new(),
			slabs_partial: IntrusiveLinkedList::new(),
			slabs_free: IntrusiveLinkedList::new(),
			object_size: size,
			slab_order,
			objects_per_slab,
		};
	}

	pub const fn object_size(&self) -> usize {
		return self.object_size;
	}
}

// Private interface
impl SlabCache {
	pub fn setup_free_list(
		&self,
		start: VirtAddr,
		count: usize,
	) -> Option<NonNull<u8>> {
		use core::ptr;

		if count == 0 || self.object_size < mem::size_of::<*mut u8>() {
			println_serial!(
                "setup_free_list: Returning None! count={}, object_size={}, pointer_size={}",
                count,
                self.object_size,
                mem::size_of::<*mut u8>()
            );
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
		return NonNull::new(start.as_mut_ptr::<u8>());
	}
}
