use super::memblock::MemBlockAllocator;
use crate::sync::locked::Locked;

#[global_allocator]
pub static ALLOCATOR: Locked<MemBlockAllocator> =
	Locked::new(MemBlockAllocator::new());
