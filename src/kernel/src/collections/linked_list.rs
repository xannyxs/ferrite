//! A doubly-linked list with owned nodes.
//!
//! The LinkedList allows pushing and popping elements at either end in
//! constant time.
//! NOTE: It is almost always better to use Vec or VecDeque
//! because array-based containers are generally faster, more memory
//! efficient, and make better use of CPU cache.

use alloc::boxed::Box;
use core::ptr::NonNull;

/// A node in a doubly-linked list.
///
/// Each node contains an element of type T and optional pointers to the next
/// and previous nodes in the list. The pointers are represented as `NonNull`
/// to ensure they're never null when present.
pub struct Node<T> {
	next: Option<NonNull<Node<T>>>,
	prev: Option<NonNull<Node<T>>>,
	/// The data stored within the node.
	pub element: T,
}

/// A doubly-linked list implementation.
///
/// This list maintains pointers to both the head and tail nodes, allowing
/// efficient operations at either end. It manually manages memory for its
/// nodes, requiring careful attention to safety constraints.
///
/// The list tracks its length, which is updated whenever nodes are added or
/// removed.
#[derive(Default)]
pub struct LinkedList<T> {
	head: Option<NonNull<Node<T>>>,
	tail: Option<NonNull<Node<T>>>,
	length: usize,
}

impl<T> Drop for LinkedList<T> {
	#[inline]
	fn drop(&mut self) {
		self.clear();
	}
}

impl<T> LinkedList<T> {
	/// Returns true if the list contains no elements, false otherwise.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn is_empty(&self) -> bool {
		self.head.is_none()
	}

	/// Returns the number of elements currently in the list.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn len(&self) -> usize {
		self.length
	}

	/// Removes all elements from the list, properly deallocating their memory.
	#[inline]
	pub fn clear(&mut self) {
		while !self.is_empty() {
			unsafe {
				self.pop_back();
			}
		}
	}

	/// Removes and returns the first element from the list, or None if empty.
	///
	/// # Safety
	/// Uses Box::from_raw to reconstruct and drop Boxes, directly manipulates
	/// raw pointers.
	pub unsafe fn pop_front(&mut self) -> Option<T> {
		if self.length == 0 {
			return None;
		}

		let head_ptr = self.head?;

		if self.length == 1 {
			self.tail = None;
			self.head = None;
			self.length = 0;

			let head_box = unsafe { Box::from_raw(head_ptr.as_ptr()) };
			return Some(head_box.element);
		}

		let mut second_ptr = unsafe { head_ptr.as_ref().next? };

		unsafe {
			second_ptr.as_mut().prev = None;
		}

		self.head = Some(second_ptr);
		self.length -= 1;

		let head_box = unsafe { Box::from_raw(head_ptr.as_ptr()) };

		return Some(head_box.element);
	}

	/// Removes and returns the last element from the list, or None if empty.
	///
	/// # Safety
	/// Uses Box::from_raw to reconstruct and drop Boxes, directly manipulates
	/// raw pointers.
	pub unsafe fn pop_back(&mut self) -> Option<T> {
		if self.length == 0 {
			return None;
		}

		let tail_ptr = self.tail?;

		if self.length == 1 {
			self.tail = None;
			self.head = None;
			self.length = 0;

			let tail_box = unsafe { Box::from_raw(tail_ptr.as_ptr()) };
			return Some(tail_box.element);
		}

		let mut second_last_ptr = unsafe { tail_ptr.as_ref().prev? };

		unsafe {
			second_last_ptr.as_mut().next = None;
		}

		self.tail = Some(second_last_ptr);
		self.length -= 1;

		let tail_box = unsafe { Box::from_raw(tail_ptr.as_ptr()) };

		return Some(tail_box.element);
	}

	/// Adds a node to the front of the list.
	///
	/// # Safety
	/// - Uses `Box::leak` to manually manage memory that must be freed in
	///   `drop`
	/// - Manipulates raw pointers which must remain valid for the list's
	///   lifetime
	/// - Caller must ensure the list is properly initialized and eventually
	///   dropped
	pub unsafe fn push_front(&mut self, element: T) {
		let new_node = Box::new(Node {
			element,
			next: self.head,
			prev: None,
		});

		let node_ptr = NonNull::from(Box::leak(new_node));

		match self.head {
			Some(mut head_ptr) => unsafe {
				head_ptr.as_mut().prev = Some(node_ptr);
			},
			None => {
				self.tail = Some(node_ptr);
			}
		}

		self.head = Some(node_ptr);
		self.length += 1;
	}

	/// Adds a node to the back of the list.
	///
	/// # Safety
	/// - Uses `Box::leak` to manually manage memory that must be freed in
	///   `drop`
	/// - Manipulates raw pointers which must remain valid for the list's
	///   lifetime
	/// - Caller must ensure the list is properly initialized and eventually
	///   dropped
	pub unsafe fn push_back(&mut self, element: T) {
		let new_node = Box::new(Node {
			element,
			next: None,
			prev: self.tail,
		});

		let node_ptr = NonNull::from(Box::leak(new_node));

		match self.tail {
			Some(mut tail_ptr) => unsafe {
				tail_ptr.as_mut().next = Some(node_ptr);
			},
			None => {
				self.head = Some(node_ptr);
			}
		}
		self.tail = Some(node_ptr);
		self.length += 1;
	}

	/// Returns a reference to the first element in the list, or None if the
	/// list is empty.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn front(&self) -> Option<&T> {
		unsafe { self.head.as_ref().map(|node| &node.as_ref().element) }
	}

	/// Returns a mutable reference to the first element in the list, or None if
	/// the list is empty.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn front_mut(&mut self) -> Option<&mut T> {
		unsafe { self.head.as_mut().map(|node| &mut node.as_mut().element) }
	}

	/// Returns a reference to the last element in the list, or None if the list
	/// is empty.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn back(&self) -> Option<&T> {
		unsafe { self.tail.as_ref().map(|node| &node.as_ref().element) }
	}

	/// Returns a mutable reference to the last element in the list, or None if
	/// the list is empty.
	#[inline]
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn back_mut(&mut self) -> Option<&mut T> {
		unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().element) }
	}
}
