use core::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PhysAddr(usize);

impl From<usize> for PhysAddr {
	fn from(addr: usize) -> Self {
		PhysAddr(addr)
	}
}
impl From<PhysAddr> for usize {
	fn from(va: PhysAddr) -> Self {
		va.0
	}
}

impl Add<usize> for PhysAddr {
	type Output = Self;

	#[inline]
	fn add(self, rhs: usize) -> Self::Output {
		PhysAddr::new(self.0.checked_add(rhs).unwrap())
	}
}

impl AddAssign<usize> for PhysAddr {
	#[inline]
	fn add_assign(&mut self, rhs: usize) {
		*self = *self + rhs;
	}
}

impl Sub<usize> for PhysAddr {
	type Output = Self;

	#[inline]
	fn sub(self, rhs: usize) -> Self::Output {
		PhysAddr::new(self.0.checked_sub(rhs).unwrap())
	}
}

impl Sub<PhysAddr> for PhysAddr {
	type Output = usize;

	#[inline]
	fn sub(self, rhs: PhysAddr) -> Self::Output {
		self.as_usize().checked_sub(rhs.as_usize()).unwrap()
	}
}

impl PhysAddr {
	#[inline]
	#[must_use]
	pub const fn new(addr: usize) -> Self {
		Self(addr)
	}

	#[inline]
	#[must_use]
	pub const fn as_usize(self) -> usize {
		self.0
	}
}

/* -------------------------------------- */

/// Type alias for representing virtual memory addresses.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtAddr(usize);

impl From<usize> for VirtAddr {
	fn from(addr: usize) -> Self {
		VirtAddr(addr)
	}
}
impl From<VirtAddr> for usize {
	fn from(va: VirtAddr) -> Self {
		va.0
	}
}

impl Add<usize> for VirtAddr {
	type Output = Self;

	#[inline]
	fn add(self, rhs: usize) -> Self::Output {
		VirtAddr::new(self.0.checked_add(rhs).unwrap())
	}
}

impl VirtAddr {
	#[inline]
	#[must_use]
	pub const fn new(addr: usize) -> VirtAddr {
		VirtAddr(addr)
	}

	#[inline]
	#[must_use]
	pub const fn as_usize(self) -> usize {
		self.0
	}

	#[inline]
	#[must_use]
	pub fn as_ptr<T>(self) -> *const T {
		core::ptr::with_exposed_provenance(self.0)
	}

	#[inline]
	#[must_use]
	pub fn as_mut_ptr<T>(self) -> *mut T {
		core::ptr::with_exposed_provenance_mut(self.0)
	}
}
