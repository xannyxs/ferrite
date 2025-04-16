use core::{
	borrow::{Borrow, BorrowMut},
	marker::PhantomData,
	ptr::{self, NonNull},
};

#[derive(Debug)]
pub struct IntrusiveNode<T: ?Sized> {
	container: Option<NonNull<T>>,
	next: Option<NonNull<IntrusiveNode<T>>>,
	prev: Option<NonNull<IntrusiveNode<T>>>,
	_marker: core::marker::PhantomData<T>,
}

impl<T: ?Sized> Default for IntrusiveNode<T> {
	#[inline]
	fn default() -> Self {
		return Self::new(None);
	}
}

impl<T: ?Sized> IntrusiveNode<T> {
	pub const fn new(container: Option<NonNull<T>>) -> Self {
		return Self {
			container,
			next: None,
			prev: None,
			_marker: PhantomData,
		};
	}

	/// Returns an optional shared reference to the container struct (`T`)
	/// this node is embedded within.
	/// Returns `None` if the back-pointer was not set or is None.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn container(&self) -> Option<&T> {
		self.container
			.map(|ptr_nn: NonNull<T>| unsafe { ptr_nn.as_ref() })
	}

	/// Returns an optional mutable reference to the container struct (`T`)
	/// this node is embedded within.
	/// Returns `None` if the back-pointer was not set or is None.
	///
	/// # Safety
	/// Caller must ensure unique mutable access according to borrowing rules.
	#[inline]
	#[must_use]
	pub unsafe fn container_mut(&mut self) -> Option<&mut T> {
		self.container.map(|mut ptr| unsafe { ptr.as_mut() })
	}
}

pub struct IntrusiveLinkedList<T: ?Sized> {
	head: Option<NonNull<IntrusiveNode<T>>>,
	tail: Option<NonNull<IntrusiveNode<T>>>,
	len: usize,
}

// Public Interface
impl<T: ?Sized> IntrusiveLinkedList<T> {
	pub const fn new() -> Self {
		return Self {
			head: None,
			tail: None,
			len: 0,
		};
	}

	/// Returns `true` if the `LinkedList` is empty.
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		return self.head.is_none();
	}

	pub fn pop_front(&mut self) -> Option<NonNull<IntrusiveNode<T>>> {
		return self.pop_front_node();
	}

	#[allow(clippy::unwrap_used)]
	pub fn push_front(&mut self, ptr: Option<NonNull<IntrusiveNode<T>>>) {
		unsafe { self.push_front_node(ptr.unwrap()) };
	}

	pub fn pop_back(&mut self) -> Option<NonNull<IntrusiveNode<T>>> {
		return self.pop_back_node();
	}

	#[allow(clippy::unwrap_used)]
	pub fn push_back(&mut self, ptr: Option<NonNull<IntrusiveNode<T>>>) {
		unsafe { self.push_back_node(ptr.unwrap()) };
	}

	/// Returns a reference to the first element in the list, or None if the
	/// list is empty.
	#[allow(clippy::implicit_return)]
	pub fn front(&self) -> Option<&IntrusiveNode<T>> {
		self.head.map(|node_ptr| unsafe { node_ptr.as_ref() })
	}

	/// Returns a mutable reference to the first element in the list, or None if
	/// the list is empty.
	#[allow(clippy::implicit_return)]
	pub fn front_mut(&mut self) -> Option<&mut IntrusiveNode<T>> {
		self.head.map(|mut node_ptr| unsafe { node_ptr.as_mut() })
	}

	/// Returns a reference to the last element in the list, or None if the list
	/// is empty.
	#[allow(clippy::implicit_return)]
	pub fn back(&self) -> Option<&IntrusiveNode<T>> {
		self.tail.map(|node_ptr| unsafe { node_ptr.as_ref() })
	}

	/// Returns a mutable reference to the last element in the list, or None if
	/// the list is empty.
	#[allow(clippy::implicit_return)]
	pub fn back_mut(&mut self) -> Option<&mut IntrusiveNode<T>> {
		self.tail.map(|mut node_ptr| unsafe { node_ptr.as_mut() })
	}
}

// Private Interface
impl<T: ?Sized> IntrusiveLinkedList<T> {
	fn pop_front_node(&mut self) -> Option<NonNull<IntrusiveNode<T>>> {
		let mut popped_node_ptr: NonNull<IntrusiveNode<T>> =
			self.head.take()?;
		let popped_node = unsafe { popped_node_ptr.as_mut() };
		let new_head_option = popped_node.next.take();
		self.head = new_head_option;

		match self.head {
			Some(mut new_head_ptr) => {
				unsafe { new_head_ptr.as_mut().prev = None };
			}
			None => {
				self.tail = None;
			}
		}

		popped_node.prev = None;
		self.len -= 1;

		return Some(popped_node_ptr);
	}

	/// Pushes a node (via NonNull pointer to its IntrusiveNode) onto the front
	/// of the list.
	///
	/// # Safety
	/// - `node_ptr` MUST point to a valid IntrusiveNode<T> within a T that has
	///   a stable memory location.
	/// - The node must not already be in this list.
	/// - Caller must ensure synchronization if used concurrently.
	pub unsafe fn push_front_node(
		&mut self,
		mut node_ptr: NonNull<IntrusiveNode<T>>,
	) {
		let node = unsafe { node_ptr.as_mut() };

		node.next = self.head;
		node.prev = None;

		match self.head {
			None => {
				self.tail = Some(node_ptr);
			}
			Some(mut old_head_ptr) => {
				let old_head_node = unsafe { old_head_ptr.as_mut() };
				old_head_node.prev = Some(node_ptr);
			}
		}

		self.head = Some(node_ptr);
		self.len += 1;
	}

	fn pop_back_node(&mut self) -> Option<NonNull<IntrusiveNode<T>>> {
		let mut popped_node_ptr: NonNull<IntrusiveNode<T>> =
			self.tail.take()?;
		let popped_node = unsafe { popped_node_ptr.as_mut() };
		let new_tail_option = popped_node.prev.take();
		self.tail = new_tail_option;

		match self.tail {
			Some(mut new_tail_ptr) => {
				unsafe { new_tail_ptr.as_mut().next = None };
			}
			None => {
				self.head = None;
			}
		}

		popped_node.next = None;
		self.len -= 1;

		return Some(popped_node_ptr);
	}

	/// Pushes a node (via NonNull pointer to its IntrusiveNode) onto the front
	/// of the list.
	///
	/// # Safety
	/// - `node_ptr` MUST point to a valid IntrusiveNode<T> within a T that has
	///   a stable memory location.
	/// - The node must not already be in this list.
	/// - Caller must ensure synchronization if used concurrently.
	pub unsafe fn push_back_node(
		&mut self,
		mut node_ptr: NonNull<IntrusiveNode<T>>,
	) {
		let node = unsafe { node_ptr.as_mut() };

		node.prev = self.tail;
		node.next = None;

		match self.tail {
			None => {
				self.head = Some(node_ptr);
			}
			Some(mut old_head_ptr) => {
				let old_head_node = unsafe { old_head_ptr.as_mut() };
				old_head_node.next = Some(node_ptr);
			}
		}

		self.tail = Some(node_ptr);
		self.len += 1;
	}
}
