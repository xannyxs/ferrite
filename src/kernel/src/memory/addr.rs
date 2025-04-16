use core::ops::{Add, Sub};

/// Represents a physical memory address, wrapping a `usize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PhysAddr(usize);

// --- Trait Implementations for PhysAddr ---

impl From<usize> for PhysAddr {
	/// Creates a `PhysAddr` directly from a `usize` value.
	#[inline]
	fn from(addr: usize) -> Self {
		return PhysAddr(addr);
	}
}

impl From<PhysAddr> for usize {
	/// Converts a `PhysAddr` back into its underlying `usize` value.
	#[inline]
	fn from(pa: PhysAddr) -> Self {
		return pa.0;
	}
}

impl Add<usize> for PhysAddr {
	type Output = Self;

	/// Adds a `usize` offset to the physical address.
	/// Panics on overflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn add(self, rhs: usize) -> Self::Output {
		return PhysAddr::new(
			self.0.checked_add(rhs).expect("PhysAddr add overflow"),
		);
	}
}

impl Sub<PhysAddr> for PhysAddr {
	type Output = usize;

	/// Calculates the difference (offset) between two physical addresses.
	/// Panics on underflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn sub(self, rhs: PhysAddr) -> Self::Output {
		return self
			.as_usize()
			.checked_sub(rhs.as_usize())
			.expect("PhysAddr sub underflow");
	}
}

// --- Inherent Methods for PhysAddr ---

impl PhysAddr {
	/// Creates a new `PhysAddr` from a `usize`. (const version)
	#[inline]
	#[must_use]
	pub const fn new(addr: usize) -> Self {
		return Self(addr);
	}

	/// Returns the underlying `usize` representation of the physical address.
	/// (const version)
	#[inline]
	#[must_use]
	pub const fn as_usize(self) -> usize {
		return self.0;
	}

	/// Converts the physical address into a raw constant pointer of type `T`.
	/// Note: Dereferencing requires `unsafe` and ensuring the address is valid.
	#[inline]
	#[must_use]
	pub fn as_ptr<T>(self) -> *const T {
		return core::ptr::with_exposed_provenance(self.0);
	}

	/// Converts the physical address into a raw mutable pointer of type `T`.
	/// Note: Dereferencing requires `unsafe` and ensuring the address is valid
	/// and mapped R/W.
	#[inline]
	#[must_use]
	pub fn as_mut_ptr<T>(self) -> *mut T {
		return core::ptr::with_exposed_provenance_mut(self.0);
	}
}

/* -------------------------------------- */

/// Represents a virtual memory address, wrapping a `usize`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)] // Added Debug derive
#[repr(transparent)]
pub struct VirtAddr(usize);

// --- Trait Implementations for VirtAddr ---

impl From<usize> for VirtAddr {
	/// Creates a `VirtAddr` directly from a `usize` value.
	#[inline]
	fn from(addr: usize) -> Self {
		// TODO: Consider adding checks for canonical address range if needed
		return VirtAddr(addr);
	}
}

impl From<VirtAddr> for usize {
	/// Converts a `VirtAddr` back into its underlying `usize` value.
	#[inline]
	fn from(va: VirtAddr) -> Self {
		return va.0;
	}
}

impl Add<usize> for VirtAddr {
	type Output = Self;

	/// Adds a `usize` offset to the virtual address.
	/// Panics on overflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn add(self, rhs: usize) -> Self::Output {
		// Use constructor for type safety
		return VirtAddr::new(
			self.0.checked_add(rhs).expect("VirtAddr add overflow"),
		);
	}
}

impl Sub<VirtAddr> for VirtAddr {
	type Output = usize;

	/// Calculates the difference (offset) between two virtual addresses.
	/// Panics on underflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn sub(self, rhs: VirtAddr) -> Self::Output {
		return self
			.as_usize()
			.checked_sub(rhs.as_usize())
			.expect("VirtAddr sub underflow");
	}
}

// --- Inherent Methods for VirtAddr ---

impl VirtAddr {
	/// Creates a new `VirtAddr` from a `usize`. (const version)
	#[inline]
	#[must_use]
	pub const fn new(addr: usize) -> VirtAddr {
		// TODO: Consider adding checks for canonical address range if needed
		return VirtAddr(addr);
	}

	/// Returns the underlying `usize` representation of the virtual address.
	#[inline]
	#[must_use]
	pub const fn as_usize(self) -> usize {
		return self.0;
	}

	/// Converts the virtual address into a raw constant pointer of type `T`.
	/// Note: Dereferencing requires `unsafe`.
	#[inline]
	#[must_use]
	pub fn as_ptr<T>(self) -> *const T {
		return core::ptr::with_exposed_provenance(self.0);
	}

	/// Converts the virtual address into a raw mutable pointer of type `T`.
	/// Note: Dereferencing requires `unsafe`.
	#[inline]
	#[must_use]
	pub fn as_mut_ptr<T>(self) -> *mut T {
		return core::ptr::with_exposed_provenance_mut(self.0);
	}
}
