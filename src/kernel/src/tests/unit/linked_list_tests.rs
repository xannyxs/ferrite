use super::*;
use crate::collections::linked_list::LinkedList;
use alloc::vec::Vec;

// Helper function to create a list with some values
fn create_test_list() -> LinkedList<i32> {
	let mut list = LinkedList::default();
	unsafe {
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
	}

	return list;
}

#[test_case]
fn test_new_list_is_empty() {
	let list: LinkedList<i32> = LinkedList::default();
	assert!(list.is_empty());
	assert_eq!(list.len(), 0);
	assert!(list.front().is_none());
	assert!(list.back().is_none());
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_push_back() {
	let mut list = LinkedList::default();

	// Add first element
	unsafe { list.push_back(10) };
	assert_eq!(list.len(), 1);
	assert_eq!(*list.front().unwrap(), 10);
	assert_eq!(*list.back().unwrap(), 10);

	// Add second element
	unsafe { list.push_back(20) };
	assert_eq!(list.len(), 2);
	assert_eq!(*list.front().unwrap(), 10);
	assert_eq!(*list.back().unwrap(), 20);

	// Add third element
	unsafe { list.push_back(30) };
	assert_eq!(list.len(), 3);
	assert_eq!(*list.front().unwrap(), 10);
	assert_eq!(*list.back().unwrap(), 30);
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_push_front() {
	let mut list = LinkedList::default();

	// Add first element
	unsafe { list.push_front(10) };
	assert_eq!(list.len(), 1);
	assert_eq!(*list.front().unwrap(), 10);
	assert_eq!(*list.back().unwrap(), 10);

	// Add second element
	/* unsafe { list.push_front(20) };
	assert_eq!(list.len(), 2);
	assert_eq!(*list.front().unwrap(), 20);
	assert_eq!(*list.back().unwrap(), 10); */

	// Add third element
	/* unsafe { list.push_front(30) };
	assert_eq!(list.len(), 3);
	assert_eq!(*list.front().unwrap(), 30);
	assert_eq!(*list.back().unwrap(), 10); */
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_pop_back() {
	let mut list = create_test_list(); // List contains [1, 2, 3]

	// Remove last element
	assert_eq!(unsafe { list.pop_back() }, Some(3));
	assert_eq!(list.len(), 2);
	assert_eq!(*list.back().unwrap(), 2);

	// Remove second element
	assert_eq!(unsafe { list.pop_back() }, Some(2));
	assert_eq!(list.len(), 1);
	assert_eq!(*list.back().unwrap(), 1);
	assert_eq!(*list.front().unwrap(), 1);

	// Remove last remaining element
	assert_eq!(unsafe { list.pop_back() }, Some(1));
	assert_eq!(list.len(), 0);
	assert!(list.is_empty());
	assert!(list.back().is_none());
	assert!(list.front().is_none());

	// Try to remove from empty list
	assert_eq!(unsafe { list.pop_back() }, None);
	assert!(list.is_empty());
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_pop_front() {
	let mut list = create_test_list(); // List contains [1, 2, 3]

	// Remove first element
	assert_eq!(unsafe { list.pop_front() }, Some(1));
	assert_eq!(list.len(), 2);
	assert_eq!(*list.front().unwrap(), 2);

	// Remove second element
	assert_eq!(unsafe { list.pop_front() }, Some(2));
	assert_eq!(list.len(), 1);
	assert_eq!(*list.front().unwrap(), 3);
	assert_eq!(*list.back().unwrap(), 3);

	// Remove last remaining element
	assert_eq!(unsafe { list.pop_front() }, Some(3));
	assert_eq!(list.len(), 0);
	assert!(list.is_empty());
	assert!(list.front().is_none());
	assert!(list.back().is_none());

	// Try to remove from empty list
	assert_eq!(unsafe { list.pop_front() }, None);
	assert!(list.is_empty());
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_front_and_back_references() {
	let mut list = create_test_list(); // List contains [1, 2, 3]

	// Check front and back references
	assert_eq!(*list.front().unwrap(), 1);
	assert_eq!(*list.back().unwrap(), 3);

	// Check mutable references
	if let Some(front) = list.front_mut() {
		*front = 100;
	}
	if let Some(back) = list.back_mut() {
		*back = 300;
	}

	// Verify the changes took effect
	assert_eq!(*list.front().unwrap(), 100);
	assert_eq!(*list.back().unwrap(), 300);
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_clear() {
	let mut list = create_test_list(); // List contains [1, 2, 3]
	assert_eq!(list.len(), 3);

	list.clear();
	assert_eq!(list.len(), 0);
	assert!(list.is_empty());
	assert!(list.front().is_none());
	assert!(list.back().is_none());

	// Add after clearing to ensure the list still works
	unsafe { list.push_back(5) };
	assert_eq!(list.len(), 1);
	assert_eq!(*list.front().unwrap(), 5);
}

#[test_case]
#[allow(clippy::unwrap_used)]
fn test_push_pop_alternating() {
	let mut list = LinkedList::default();

	// Push and pop alternating to test both growing and shrinking
	unsafe { list.push_back(1) };
	assert_eq!(list.len(), 1);

	assert_eq!(unsafe { list.pop_back() }, Some(1));
	assert_eq!(list.len(), 0);

	unsafe { list.push_front(2) };
	assert_eq!(list.len(), 1);

	unsafe { list.push_back(3) };
	assert_eq!(list.len(), 2);

	assert_eq!(unsafe { list.pop_front() }, Some(2));
	assert_eq!(list.len(), 1);

	unsafe { list.push_back(4) };
	assert_eq!(list.len(), 2);

	assert_eq!(*list.front().unwrap(), 3);
	assert_eq!(*list.back().unwrap(), 4);
}

/* #[test_case]
fn test_memory_management() {
	// This test uses a custom Drop-tracking type to ensure memory is
	// properly managed
	struct DropCounter<'a> {
		value: i32,
		counter: &'a mut i32,
	}

	impl<'a> Drop for DropCounter<'a> {
		fn drop(&mut self) {
			*self.counter += 1;
		}
	}

	let mut drop_count = 0;

	{
		let mut list = LinkedList::default();

		// Add elements that increment the counter when dropped
		unsafe {
			list.push_back(DropCounter {
				value: 1,
				counter: &mut drop_count,
			});
			list.push_back(DropCounter {
				value: 2,
				counter: &mut drop_count,
			});
			list.push_back(DropCounter {
				value: 3,
				counter: &mut drop_count,
			});
		}

		// Pop one element - should trigger one drop
		let _ = unsafe { list.pop_back() };
		assert_eq!(drop_count, 1);

		// The rest should be dropped when the list goes out of scope
	}

	// Check that all remaining elements were properly dropped
	assert_eq!(drop_count, 3);
} */
