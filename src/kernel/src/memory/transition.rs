use crate::memory::allocator::{
	BUDDY_PAGE_ALLOCATOR, EARLY_PHYSICAL_ALLOCATOR,
};

fn transition_allocator() {
	let memblock = EARLY_PHYSICAL_ALLOCATOR.lock();
	let mut buddy_allocator = BUDDY_PAGE_ALLOCATOR.lock();

	unimplemented!();
}
