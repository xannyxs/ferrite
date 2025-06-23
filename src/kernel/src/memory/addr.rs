//! The PhysAddr & VirtAddr to easily convert addresses and represent their
//! address type

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
		PhysAddr(addr)
	}
}

impl From<PhysAddr> for usize {
	/// Converts a `PhysAddr` back into its underlying `usize` value.
	#[inline]
	fn from(pa: PhysAddr) -> Self {
		pa.0
	}
}

impl Add<usize> for PhysAddr {
	type Output = Self;

	/// Adds a `usize` offset to the physical address.
	/// Panics on overflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn add(self, rhs: usize) -> Self::Output {
		PhysAddr::new(self.0.checked_add(rhs).expect("PhysAddr add overflow"))
	}
}

impl Sub<PhysAddr> for PhysAddr {
	type Output = usize;

	/// Calculates the difference (offset) between two physical addresses.
	/// Panics on underflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn sub(self, rhs: PhysAddr) -> Self::Output {
		self.as_usize()
			.checked_sub(rhs.as_usize())
			.expect("PhysAddr sub underflow")
	}
}

// --- Inherent Methods for PhysAddr ---

impl PhysAddr {
	/// Creates a new `PhysAddr` from a `usize`. (const version)
	#[inline]
	#[must_use]
	pub const fn new(addr: usize) -> Self {
		Self(addr)
	}

	/// Returns the underlying `usize` representation of the physical address.
	/// (const version)
	#[inline]
	#[must_use]
	pub const fn as_usize(self) -> usize {
		self.0
	}

	/// Converts the physical address into a raw constant pointer of type `T`.
	/// Note: Dereferencing requires `unsafe` and ensuring the address is valid.
	#[inline]
	#[must_use]
	pub fn as_ptr<T>(self) -> *const T {
		core::ptr::with_exposed_provenance(self.0)
	}

	/// Converts the physical address into a raw mutable pointer of type `T`.
	/// Note: Dereferencing requires `unsafe` and ensuring the address is valid
	/// and mapped R/W.
	#[inline]
	#[must_use]
	pub fn as_mut_ptr<T>(self) -> *mut T {
		core::ptr::with_exposed_provenance_mut(self.0)
	}

	/// Aligns the physical address upwards to the given alignment.
	///
	/// See the `align_up` function for more information.
	///
	/// # Panics
	///
	/// This function panics if the resulting address has a bit in the range 52
	/// to 64 set.
	#[inline]
	pub fn align_up<U>(self, align: U) -> Self
	where
		U: Into<usize>,
	{
		PhysAddr::new(align_up(self.0, align.into()))
	}

	/// Aligns the physical address downwards to the given alignment.
	///
	/// See the `align_down` function for more information.
	#[inline]
	pub fn align_down<U>(self, align: U) -> Self
	where
		U: Into<usize>,
	{
		self.align_down_usize(align.into())
	}

	/// Aligns the physical address downwards to the given alignment.
	///
	/// See the `align_down` function for more information.
	#[inline]
	pub(crate) const fn align_down_usize(self, align: usize) -> Self {
		PhysAddr(align_down(self.0, align))
	}

	/// Checks whether the physical address has the demanded alignment.
	#[inline]
	pub fn is_aligned<U>(self, align: U) -> bool
	where
		U: Into<usize>,
	{
		self.is_aligned_usize(align.into())
	}

	/// Checks whether the physical address has the demanded alignment.
	#[inline]
	pub(crate) const fn is_aligned_usize(self, align: usize) -> bool {
		self.align_down_usize(align).as_usize() == self.as_usize()
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
		VirtAddr(addr)
	}
}

impl From<VirtAddr> for usize {
	/// Converts a `VirtAddr` back into its underlying `usize` value.
	#[inline]
	fn from(va: VirtAddr) -> Self {
		va.0
	}
}

impl Add<usize> for VirtAddr {
	type Output = Self;

	/// Adds a `usize` offset to the virtual address.
	/// Panics on overflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn add(self, rhs: usize) -> Self::Output {
		VirtAddr::new(self.0.checked_add(rhs).expect("VirtAddr add overflow"))
	}
}

impl Sub<VirtAddr> for VirtAddr {
	type Output = usize;

	/// Calculates the difference (offset) between two virtual addresses.
	/// Panics on underflow in debug builds (due to unwrap).
	#[inline]
	#[allow(clippy::expect_used)]
	fn sub(self, rhs: VirtAddr) -> Self::Output {
		self.as_usize()
			.checked_sub(rhs.as_usize())
			.expect("VirtAddr sub underflow")
	}
}

// --- Inherent Methods for VirtAddr ---

impl VirtAddr {
	/// Creates a new `VirtAddr` from a `usize`. (const version)
	#[inline]
	#[must_use]
	pub const fn new(addr: usize) -> VirtAddr {
		// TODO: Consider adding checks for canonical address range if needed
		VirtAddr(addr)
	}

	/// Creates a new canonical virtual address, throwing out bits 24..32.
	///
	/// This function performs sign extension of bit 47 to make the address
	/// canonical, overwriting bits 48 to 64. If you want to check whether an
	/// address is canonical, use [`new`](Self::new) or
	/// [`try_new`](Self::try_new).
	#[inline]
	pub const fn new_truncate(addr: usize) -> VirtAddr {
		VirtAddr(((addr << 8) as isize >> 8) as usize)
	}

	/// Returns the underlying `usize` representation of the virtual address.
	#[inline]
	#[must_use]
	pub const fn as_usize(self) -> usize {
		self.0
	}

	/// Converts the virtual address into a raw constant pointer of type `T`.
	/// Note: Dereferencing requires `unsafe`.
	#[inline]
	#[must_use]
	pub fn as_ptr<T>(self) -> *const T {
		core::ptr::with_exposed_provenance(self.0)
	}

	/// Aligns the virtual address upwards to the given alignment.
	///
	/// See the `align_up` function for more information.
	///
	/// # Panics
	///
	/// This function panics if the resulting address is higher than
	/// `0xffff_ffff_ffff_ffff`.
	#[inline]
	pub fn align_up<U>(self, align: U) -> Self
	where
		U: Into<usize>,
	{
		let aligned_addr = align_up(self.0, align.into());

		VirtAddr::new(aligned_addr)
	}

	/// Aligns the virtual address downwards to the given alignment.
	///
	/// See the `align_down` function for more information.
	#[inline]
	pub fn align_down<U>(self, align: U) -> Self
	where
		U: Into<usize>,
	{
		self.align_down_usize(align.into())
	}

	/// Converts the virtual address into a raw mutable pointer of type `T`.
	/// Note: Dereferencing requires `unsafe`.
	#[inline]
	#[must_use]
	pub fn as_mut_ptr<T>(self) -> *mut T {
		core::ptr::with_exposed_provenance_mut(self.0)
	}

	/// Aligns the virtual address downwards to the given alignment.
	///
	/// See the `align_down` function for more information.
	#[inline]
	pub(crate) const fn align_down_usize(self, align: usize) -> Self {
		VirtAddr(align_down(self.0, align))
	}

	/// Checks whether the virtual address has the demanded alignment.
	#[inline]
	pub fn is_aligned<U>(self, align: U) -> bool
	where
		U: Into<usize>,
	{
		self.is_aligned_usize(align.into())
	}

	/// Checks whether the virtual address has the demanded alignment.
	#[inline]
	pub(crate) const fn is_aligned_usize(self, align: usize) -> bool {
		self.align_down_usize(align).as_usize() == self.as_usize()
	}
}

/// Align address downwards.
///
/// Returns the greatest `x` with alignment `align` so that `x <= addr`.
///
/// Panics if the alignment is not a power of two.
#[inline]
pub const fn align_down(addr: usize, align: usize) -> usize {
	assert!(align.is_power_of_two(), "`align` must be a power of two");
	addr & !(align - 1)
}

/// Align address upwards.
///
/// Returns the smallest `x` with alignment `align` so that `x >= addr`.
///
/// Panics if the alignment is not a power of two or if an overflow occurs.
#[inline]
pub const fn align_up(addr: usize, align: usize) -> usize {
	assert!(align.is_power_of_two(), "`align` must be a power of two");
	let align_mask = align - 1;
	if addr & align_mask == 0 {
		addr // already aligned
	} else {
		// FIXME: Replace with .expect, once `Option::expect` is const.
		if let Some(aligned) = (addr | align_mask).checked_add(1) {
			aligned
		} else {
			panic!("attempt to add with overflow")
		}
	}
}
