//! Defines the kernel's global memory allocator instance.

use super::memblock::MemBlockAllocator;
use crate::sync::Locked;

/// The kernel's global memory allocator instance.
///
/// This static variable is marked with `#[global_allocator]`, making it the
/// default allocator used by the `alloc` crate (e.g., for `Box`, `Vec`) once
/// enabled.
///
/// It wraps the early physical `MemBlockAllocator` in a `Locked` type to ensure
/// thread-safe access.
///
/// # Notes
/// Using `MemBlockAllocator` directly as the global allocator is suitable only
/// for the very early stages of kernel initialization before a proper page
/// allocator (like Buddy) and slab allocator are available. `MemBlockAllocator`
/// typically doesn't support deallocation effectively.
#[global_allocator]
pub static ALLOCATOR: Locked<MemBlockAllocator> =
	Locked::new(MemBlockAllocator::new());
