//! A doubly-linked list with owned nodes.

use alloc::{alloc::Global, boxed::Box};
use core::{alloc::Allocator, ptr::NonNull};

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

impl<T> Node<T> {
	fn new(element: T) -> Self {
		Node {
			next: None,
			prev: None,
			element,
		}
	}

	#[allow(clippy::boxed_local)]
	fn into_element<A: Allocator>(self: Box<Self, A>) -> T {
		self.element
	}
}

/// A doubly-linked list implementation.
///
/// This list maintains pointers to both the head and tail nodes, allowing
/// efficient operations at either end. It manually manages memory for its
/// nodes, requiring careful attention to safety constraints.
///
/// The list tracks its length, which is updated whenever nodes are added or
/// removed.
pub struct LinkedList<T, A: Allocator = Global> {
	head: Option<NonNull<Node<T>>>,
	tail: Option<NonNull<Node<T>>>,
	len: usize,
	alloc: A,
}

// Private methods
impl<T, A: Allocator> LinkedList<T, A> {
	/// Removes and returns the first element from the list, or None if empty.
	#[inline]
	#[allow(clippy::implicit_return)]
	fn pop_front_node(&mut self) -> Option<Box<Node<T>, &A>> {
		// This method takes care not to create mutable references to whole
		// nodes,
		// to maintain validity of aliasing pointers into `element`.
		self.head.map(|node| unsafe {
			let node = Box::from_raw_in(node.as_ptr(), &self.alloc);
			self.head = node.next;
			match self.head {
				None => self.tail = None,
				// Not creating new mutable (unique!) references overlapping
				// `element`.
				Some(head) => (*head.as_ptr()).prev = None,
			}

			self.len -= 1;

			node
		})
	}

	/// Removes and returns the node at the back of the list.
	#[inline]
	#[allow(clippy::implicit_return)]
	fn pop_back_node(&mut self) -> Option<Box<Node<T>, &A>> {
		// This method takes care not to create mutable references to whole
		// nodes,
		// to maintain validity of aliasing pointers into `element`.
		self.tail.map(|node| unsafe {
			let node = Box::from_raw_in(node.as_ptr(), &self.alloc);
			self.tail = node.prev;
			match self.tail {
				None => self.head = None,
				// Not creating new mutable (unique!) references overlapping
				// `element`.
				Some(tail) => (*tail.as_ptr()).next = None,
			}

			self.len -= 1;

			node
		})
	}

	/// Adds the given node to the front of the list.
	///
	/// # Safety
	/// `node` must point to a valid node that was boxed and leaked using the
	/// list's allocator. This method takes ownership of the node, so the
	/// pointer should not be used again.
	#[inline]
	unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
		// This method takes care not to create mutable references to whole
		// nodes, to maintain validity of aliasing pointers into `element`.
		unsafe {
			(*node.as_ptr()).next = self.head;
			(*node.as_ptr()).prev = None;
			let node = Some(node);

			match self.head {
				None => self.tail = node,
				// Not creating new mutable (unique!) references overlapping
				// `element`.
				Some(head) => (*head.as_ptr()).prev = node,
			}

			self.head = node;
			self.len += 1;
		}
	}

	/// Adds the given node to the back of the list.
	///
	/// # Safety
	/// `node` must point to a valid node that was boxed and leaked using the
	/// list's allocator.
	/// This method takes ownership of the node, so the pointer should not be
	/// used again.
	#[inline]
	unsafe fn push_back_node(&mut self, node: NonNull<Node<T>>) {
		// This method takes care not to create mutable references to whole
		// nodes,
		// to maintain validity of aliasing pointers into `element`.
		unsafe {
			(*node.as_ptr()).next = None;
			(*node.as_ptr()).prev = self.tail;
			let node = Some(node);
			match self.tail {
				None => self.head = node,
				// Not creating new mutable (unique!) references overlapping
				// `element`.
				Some(tail) => (*tail.as_ptr()).next = node,
			}

			self.tail = node;
			self.len += 1;
		}
	}

	/// Unlinks the specified node from the current list.
	///
	/// Warning: this will not check that the provided node belongs to the
	/// current list.
	///
	/// This method takes care not to create mutable references to `element`, to
	/// maintain validity of aliasing pointers.
	#[inline]
	unsafe fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
		let node = unsafe { node.as_mut() }; // this one is ours now, we can create an &mut.
									   // Not creating new mutable (unique!) references overlapping `element`.
		match node.prev {
			Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
			// this node is the head node
			None => self.head = node.next,
		};

		match node.next {
			Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
			// this node is the tail node
			None => self.tail = node.prev,
		};

		self.len -= 1;
	}
}

impl<T> Default for LinkedList<T> {
	/// Creates an empty `LinkedList<T>`.
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl<T> LinkedList<T> {
	/// Creates an empty `LinkedList`.
	///
	/// # Examples
	///
	/// ```
	/// use std::collections::LinkedList;
	///
	/// let list: LinkedList<u32> = LinkedList::new();
	/// ```
	#[inline]
	#[must_use]
	pub const fn new() -> Self {
		LinkedList {
			head: None,
			tail: None,
			len: 0,
			alloc: Global,
		}
	}
}

impl<T, A: Allocator> LinkedList<T, A> {
	/// Constructs an empty `LinkedList<T, A>`.
	///
	/// # Examples
	///
	/// ```
	/// #![feature(allocator_api)]
	///
	/// use std::{alloc::System, collections::LinkedList};
	///
	/// let list: LinkedList<u32, _> = LinkedList::new_in(System);
	/// ```
	#[inline]
	pub const fn new_in(alloc: A) -> Self {
		LinkedList {
			head: None,
			tail: None,
			len: 0,
			alloc,
		}
	}

	/// Returns `true` if the `LinkedList` is empty.
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.head.is_none()
	}

	/// Returns the length of the `LinkedList`.
	#[inline]
	#[must_use]
	pub fn len(&self) -> usize {
		self.len
	}

	/// Removes all elements from the `LinkedList`.
	#[inline]
	pub fn clear(&mut self) {
		use core::mem;

		// We need to drop the nodes while keeping self.alloc
		// We can do this by moving (head, tail, len) into a new list that
		// borrows self.alloc
		drop(LinkedList {
			head: self.head.take(),
			tail: self.tail.take(),
			len: mem::take(&mut self.len),
			alloc: &self.alloc,
		});
	}

	/// Returns a reference to the first element in the list, or None if the
	/// list is empty.
	#[allow(clippy::implicit_return)]
	pub fn front(&self) -> Option<&T> {
		self.head
			.map(|node_ptr| unsafe { &node_ptr.as_ref().element })
	}

	/// Returns a mutable reference to the first element in the list, or None if
	/// the list is empty.
	#[allow(clippy::implicit_return)]
	pub fn front_mut(&mut self) -> Option<&mut T> {
		self.head
			.map(|mut node_ptr| unsafe { &mut node_ptr.as_mut().element })
	}

	/// Returns a reference to the last element in the list, or None if the list
	/// is empty.
	#[allow(clippy::implicit_return)]
	pub fn back(&self) -> Option<&T> {
		self.tail
			.map(|node_ptr| unsafe { &node_ptr.as_ref().element })
	}

	/// Returns a mutable reference to the last element in the list, or None if
	/// the list is empty.
	#[allow(clippy::implicit_return)]
	pub fn back_mut(&mut self) -> Option<&mut T> {
		self.tail
			.map(|mut node_ptr| unsafe { &mut node_ptr.as_mut().element })
	}

	/// Removes the first element and returns it, or `None` if the list is
	/// empty.
	pub fn pop_front(&mut self) -> Option<T> {
		self.pop_front_node().map(Node::into_element)
	}

	/// Removes the last element from a list and returns it, or `None` if
	/// it is empty.
	pub fn pop_back(&mut self) -> Option<T> {
		self.pop_back_node().map(Node::into_element)
	}

	/// Appends an element to the back of a list.
	pub fn push_back(&mut self, elt: T) {
		let node = Box::new_in(Node::new(elt), &self.alloc);
		let node_ptr = NonNull::from(Box::leak(node));

		// SAFETY: node_ptr is a unique pointer to a node we boxed with
		// self.alloc and leaked
		unsafe {
			self.push_back_node(node_ptr);
		}
	}

	/// Adds an element first in the list.
	pub fn push_front(&mut self, elt: T) {
		let node = Box::new_in(Node::new(elt), &self.alloc);
		let node_ptr = NonNull::from(Box::leak(node));

		// SAFETY: node_ptr is a unique pointer to a node we boxed with
		// self.alloc and leaked
		unsafe {
			self.push_front_node(node_ptr);
		}
	}

	/// Provides a cursor with editing operations at the front element.
	///
	/// The cursor is pointing to the "ghost" non-element if the list is empty.
	#[inline]
	#[must_use]
	pub fn cursor_front_mut(&mut self) -> CursorMut<'_, T, A> {
		CursorMut {
			index: 0,
			current: self.head,
			list: self,
		}
	}
}

unsafe impl<#[may_dangle] T, A: Allocator> Drop for LinkedList<T, A> {
	fn drop(&mut self) {
		use core::mem;

		struct DropGuard<'a, T, A: Allocator>(&'a mut LinkedList<T, A>);

		impl<'a, T, A: Allocator> Drop for DropGuard<'a, T, A> {
			fn drop(&mut self) {
				// Continue the same loop we do below. This only runs when a
				// destructor has

				// panicked. If another one panics this will abort.

				while self.0.pop_front_node().is_some() {}
			}
		}

		// Wrap self so that if a destructor panics, we can try to keep looping

		let guard = DropGuard(self);

		while guard.0.pop_front_node().is_some() {}

		mem::forget(guard);
	}
}

/************************************* */

/// A cursor over a `LinkedList`.
///
/// A `Cursor` is like an iterator, except that it can freely seek
/// back-and-forth.
///
/// Cursors always rest between two elements in the list, and index in a
/// logically circular way.
/// To accommodate this, there is a "ghost" non-element that yields `None`
/// between the head and
/// tail of the list.
///
/// When created, cursors start at the front of the list, or the "ghost"
/// non-element if the list is empty.
pub struct Cursor<'a, T: 'a, A: Allocator = Global> {
	index: usize,
	current: Option<NonNull<Node<T>>>,
	list: &'a LinkedList<T, A>,
}

impl<'a, T, A: Allocator> Cursor<'a, T, A> {
	/// Returns the cursor position index within the `LinkedList`.
	///
	/// This returns `None` if the cursor is currently pointing to the
	/// "ghost" non-element.
	#[must_use]
	pub fn index(&self) -> Option<usize> {
		let _ = self.current?;
		Some(self.index)
	}

	/// Returns a reference to the element that the cursor is currently
	/// pointing to.
	///
	/// This returns `None` if the cursor is currently pointing to the
	/// "ghost" non-element.
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn current(&self) -> Option<&'a T> {
		unsafe { self.current.map(|current| &(*current.as_ptr()).element) }
	}

	/// Moves the cursor to the next element of the `LinkedList`.
	///
	/// If the cursor is pointing to the "ghost" non-element then this will move
	/// it to
	/// the first element of the `LinkedList`. If it is pointing to the last
	/// element of the `LinkedList` then this will move it to the "ghost"
	/// non-element.
	pub fn move_next(&mut self) {
		match self.current.take() {
			// We had no current element; the cursor was sitting at the start
			// position
			// Next element should be the head of the list
			None => {
				self.current = self.list.head;
				self.index = 0;
			}

			// We had a previous element, so let's go to its next
			Some(current) => unsafe {
				self.current = current.as_ref().next;
				self.index += 1;
			},
		}
	}
}

/// A cursor over a `LinkedList` with editing operations.
///
/// A `Cursor` is like an iterator, except that it can freely seek
/// back-and-forth, and can
/// safely mutate the list during iteration. This is because the lifetime of its
/// yielded
/// references is tied to its own lifetime, instead of just the underlying list.
/// This means
/// cursors cannot yield multiple elements at once.
///
/// Cursors always rest between two elements in the list, and index in a
/// logically circular way.
/// To accommodate this, there is a "ghost" non-element that yields `None`
/// between the head and
/// tail of the list.
pub struct CursorMut<'a, T: 'a, A: Allocator = Global> {
	index: usize,
	current: Option<NonNull<Node<T>>>,
	list: &'a mut LinkedList<T, A>,
}

#[allow(clippy::extra_unused_lifetimes)]
impl<'a, T, A: Allocator> CursorMut<'_, T, A> {
	/// Returns the cursor position index within the `LinkedList`.
	///
	/// This returns `None` if the cursor is currently pointing to the
	/// "ghost" non-element.
	#[must_use]
	pub fn index(&self) -> Option<usize> {
		let _ = self.current?;
		Some(self.index)
	}

	/// Moves the cursor to the next element of the `LinkedList`.
	///
	/// If the cursor is pointing to the "ghost" non-element then this will move
	/// it to
	/// the first element of the `LinkedList`. If it is pointing to the last
	/// element of the `LinkedList` then this will move it to the "ghost"
	/// non-element.
	pub fn move_next(&mut self) {
		match self.current.take() {
			// We had no current element; the cursor was sitting at the start
			// position
			// Next element should be the head of the list
			None => {
				self.current = self.list.head;
				self.index = 0;
			}

			// We had a previous element, so let's go to its next
			Some(current) => unsafe {
				self.current = current.as_ref().next;
				self.index += 1;
			},
		}
	}

	/// Returns a reference to the element that the cursor is currently
	/// pointing to.
	///
	/// This returns `None` if the cursor is currently pointing to the
	/// "ghost" non-element.
	#[must_use]
	#[allow(clippy::implicit_return)]
	pub fn current(&mut self) -> Option<&mut T> {
		unsafe { self.current.map(|current| &mut (*current.as_ptr()).element) }
	}

	/// Removes the current element from the `LinkedList`.
	///
	/// The element that was removed is returned, and the cursor is
	/// moved to point to the next element in the `LinkedList`.
	///
	/// If the cursor is currently pointing to the "ghost" non-element then no
	/// element is removed and `None` is returned.
	pub fn remove_current(&mut self) -> Option<T> {
		let unlinked_node = self.current?;

		unsafe {
			self.current = unlinked_node.as_ref().next;
			self.list.unlink_node(unlinked_node);

			let unlinked_node = Box::from_raw(unlinked_node.as_ptr());
			Some(unlinked_node.element)
		}
	}
}
