use core::{
	borrow::{Borrow, BorrowMut},
	ptr::{self, NonNull},
};

pub struct IntrusiveNode<T: ?Sized> {
	next: *mut IntrusiveNode<T>,
	prev: *mut IntrusiveNode<T>,
}

impl<T: ?Sized> Default for IntrusiveNode<T> {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl<T: ?Sized> IntrusiveNode<T> {
	pub const fn new() -> Self {
		return Self {
			next: ptr::null_mut(),
			prev: ptr::null_mut(),
		};
	}
}

pub struct IntrusiveLinkedList<T> {
	head: *mut IntrusiveNode<T>,
	tail: *mut IntrusiveNode<T>,
	len: usize,
}

// Public Interface
impl<T> IntrusiveLinkedList<T> {
	pub const fn new() -> Self {
		return Self {
			head: ptr::null_mut(),
			tail: ptr::null_mut(),
			len: 0,
		};
	}

	/// Returns `true` if the `LinkedList` is empty.
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		return self.head.is_null();
	}

	pub fn pop_front(&mut self) -> Option<T> {
		unimplemented!()
	}

	pub fn push_front(&mut self, ptr: *mut IntrusiveNode<T>) {
		unsafe { self.push_front_ptr(ptr) };
	}

	pub fn pop_back(&mut self) -> Option<T> {
		unimplemented!()
	}

	pub fn push_back(&mut self) {
		unimplemented!()
	}
}

// Private Interface
impl<T> IntrusiveLinkedList<T> {
	unsafe fn pop_front_ptr(&mut self) -> Option<T> {
		unimplemented!()
	}

	unsafe fn push_front_ptr(&mut self, node: *mut IntrusiveNode<T>) {
		unsafe {
			(*node).next = self.head;
			(*node).prev = ptr::null_mut();

			if !self.head.is_null() {
				(*self.head).prev = node;
			} else {
				self.tail = node;
			}
		}
		self.head = node;
		self.len += 1;
	}

	unsafe fn pop_back_ptr(&mut self, ptr: NonNull<T>) -> Option<T> {
		unimplemented!()
	}

	unsafe fn push_back_ptr(&mut self, ptr: NonNull<T>) {
		unimplemented!()
	}
}
