use super::{FrameAllocator, PhysAddr, VirtAddr, KERNEL_OFFSET};
use crate::{
	arch::x86::cpu::{cr3, invlpg},
	log_debug,
	memory::{frame::FRAME_ALLOCATOR, PAGE_SIZE},
	println_serial,
};
use core::arch::asm;

const PAGE_SIZE_4MIB: usize = 4 * 1024 * 1024;

const PDE_PRESENT: u32 = 1 << 0;
const PDE_WRITABLE: u32 = 1 << 1;
const PDE_USER: u32 = 1 << 2;
const PDE_PSE: u32 = 1 << 7;

const ADDR_MASK_4KIB_PTE: u32 = 0xfffff000;
const ADDR_MASK_4MIB_PDE: u32 = 0xffc00000;
const ADDR_MASK_PDE_TO_PT: u32 = 0xfffff000;

pub mod flags {
	pub const PRESENT: u32 = 1 << 0;
	pub const WRITABLE: u32 = 1 << 1;
	pub const USER_ACCESSIBLE: u32 = 1 << 2;
	pub const PAGE_SIZE_EXT: u32 = 1 << 7;
}

#[inline]
#[allow(clippy::expect_used)]
pub fn map_page(phys_addr: PhysAddr, virt_addr: VirtAddr, flags: u32) {
	use core::ptr;

	assert!(phys_addr.is_aligned(PAGE_SIZE));
	assert!(virt_addr.is_aligned(PAGE_SIZE));

	let vaddr = virt_addr.as_usize();
	let paddr = phys_addr.as_usize();

	let pd_paddr = cr3();
	let pd_vaddr = phys_to_virt(pd_paddr);

	let page_directory: &mut [u32; 1024] =
		unsafe { &mut *(pd_vaddr.as_mut_ptr()) };
	let pde_ref = &mut page_directory[vaddr >> 22];

	let pt_phys_addr: PhysAddr;
	if (*pde_ref & flags::PRESENT) == 0 {
		let new_pt_frame = FRAME_ALLOCATOR
            .lock()
            .get()
            .expect("Frame has not been initialized yet")
            .allocate_frame()
            .expect("Allocation Failed: Could not allocate frame for new page table");

		pt_phys_addr = new_pt_frame;
		let new_pt_virt_addr = phys_to_virt(new_pt_frame);

		let new_page_table: &mut [u32; 1024] =
			unsafe { &mut *(new_pt_virt_addr.as_mut_ptr()) };
		new_page_table.iter_mut().for_each(|entry| *entry = 0);

		*pde_ref =
			(new_pt_frame.as_usize() as u32) | flags::PRESENT | flags::WRITABLE; // Set PRESENT and WRITABLE for the PDE
	} else if (*pde_ref & flags::PAGE_SIZE_EXT) != 0 {
		panic!(
			"Conflict: Tried to map 4KiB page into a 4MiB mapped region: {:#x}",
			virt_addr.as_usize()
		);
	} else {
		pt_phys_addr = PhysAddr::new((*pde_ref & ADDR_MASK_PDE_TO_PT) as usize);
	}

	let pt_virt_addr = phys_to_virt(pt_phys_addr);
	let page_table: &mut [u32; 1024] =
		unsafe { &mut *(pt_virt_addr.as_mut_ptr()) };

	let pte_ref = &mut page_table[(vaddr >> 12) & 0x3ff];

	*pte_ref = (paddr as u32) | (flags & 0xfff) | flags::PRESENT;

	invlpg(virt_addr);
}

#[inline]
pub fn unmap_page(virt_addr: VirtAddr) {
	use core::ptr;

	assert!(virt_addr.is_aligned(PAGE_SIZE));

	let pd_paddr = cr3();
	let pd_vaddr = phys_to_virt(pd_paddr);

	let page_directory: &mut [u32; 1024] =
		unsafe { &mut *(pd_vaddr.as_mut_ptr()) };

	let pde_index = virt_addr.as_usize() >> 22;
	let pde_ref = &mut page_directory[pde_index];

	if (*pde_ref & flags::PRESENT) == 0 {
		panic!("Attempted to unmap unmapped virtual address (PDE not present): {:#x}", virt_addr.as_usize());
	}

	if (*pde_ref & flags::PAGE_SIZE_EXT) != 0 {
		panic!(
			"Attempted to unmap 4MiB page using 4KiB unmap function: {:#x}",
			virt_addr.as_usize()
		);
	}

	let pt_phys_addr = PhysAddr::new((*pde_ref & ADDR_MASK_PDE_TO_PT) as usize);
	let pt_virt_addr = phys_to_virt(pt_phys_addr);

	let page_table: &mut [u32; 1024] =
		unsafe { &mut *(pt_virt_addr.as_mut_ptr()) };

	let pte_index = (virt_addr.as_usize() >> 12) & 0x3ff;
	let pte_ref = &mut page_table[pte_index];
	let pte = *pte_ref;

	if (pte & flags::PRESENT) == 0 {
		panic!("Attempted to unmap unmapped virtual address (PTE not present): {:#x}", virt_addr.as_usize());
	}

	let mapped_frame_phys_addr =
		PhysAddr::new((pte & ADDR_MASK_4KIB_PTE) as usize);

	*pte_ref = 0;

	invlpg(virt_addr);

	FRAME_ALLOCATOR
		.lock()
		.get()
		.expect("Frame has not been initialized yet")
		.deallocate_frame(mapped_frame_phys_addr);

	let mut page_table_is_empty = true;
	for i in 0..1024 {
		if (page_table[i] & flags::PRESENT) != 0 {
			page_table_is_empty = false;
			break;
		}
	}

	if page_table_is_empty {
		FRAME_ALLOCATOR
			.lock()
			.get()
			.expect("Frame allocator not initialized for PT deallocation")
			.deallocate_frame(pt_phys_addr);

		*pde_ref = 0;
	}
}

#[inline]
#[must_use]
pub fn translate(virt_addr: VirtAddr) -> Option<PhysAddr> {
	use core::ptr;

	let paddr = cr3();
	let pd_virt_addr = phys_to_virt(paddr);
	let page_directory = unsafe { &*(pd_virt_addr.as_ptr()) as &[u32; 1024] };
	let pde = page_directory[virt_addr.as_usize() >> 22];

	if (pde & flags::PRESENT) == 0 {
		return None;
	}

	if (pde & flags::PAGE_SIZE_EXT) != 0 {
		let page_base_phys = (pde & ADDR_MASK_4MIB_PDE) as usize;
		let offset_in_page = virt_addr.as_usize() & (PAGE_SIZE_4MIB - 1);

		log_debug!(
			"4MiB page mapping: VA {:#x} -> PA {:#x}",
			virt_addr.as_usize(),
			page_base_phys + offset_in_page
		);
		return Some(PhysAddr::new(page_base_phys + offset_in_page));
	}

	let pt_phys_addr = PhysAddr::new((pde & ADDR_MASK_PDE_TO_PT) as usize);
	let pt_virt_addr = phys_to_virt(pt_phys_addr);

	let page_table =
		unsafe { &*((pt_virt_addr.as_ptr()) as *const [u32; 1024]) };
	let pte = page_table[(virt_addr.as_usize() >> 12) & 0x3ff];

	if (pte & flags::PRESENT) == 0 {
		return None;
	}

	let frame_phys_addr = (pte & ADDR_MASK_4KIB_PTE) as usize;
	let offset_in_page = virt_addr.as_usize() & (PAGE_SIZE - 1);

	log_debug!(
		"4KiB page mapping: VA {:#x} -> PA {:#x}",
		virt_addr.as_usize(),
		frame_phys_addr + offset_in_page
	);
	Some(PhysAddr::new(frame_phys_addr + offset_in_page))
}

pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
	VirtAddr::new(paddr.as_usize() + KERNEL_OFFSET)
}
