use super::{PhysAddr, VirtAddr};

pub struct Mapper;

// Public Interface
impl Mapper {
	#[inline]
	#[must_use]
	pub fn map(virt_addr: VirtAddr) {
		!unimplemented!()
	}

	#[inline]
	#[must_use]
	pub fn unmap(virt_addr: VirtAddr) {
		!unimplemented!()
	}

	#[inline]
	#[must_use]
	pub fn translate(virt_addr: VirtAddr) -> Option<PhysAddr> {
		!unimplemented!()
	}
}

// Private Interface
impl Mapper {}
