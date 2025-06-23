use crate::{log_debug, memory::paging::translate, println_serial};
use alloc::{boxed::Box, vec};

#[test_case]
fn test_translate_1() {
	assert_eq!(translate(0xc0000000.into()).unwrap().as_usize(), 0x0);
}

#[test_case]
fn test_translate_2() {
	assert_eq!(translate(0xc0400000.into()).unwrap().as_usize(), 0x00400000);
}

#[test_case]
fn test_translate_3() {
	assert_eq!(translate(0xd0000000.into()), None);
}

#[test_case]
fn test_global_allocator_simple() {
	log_debug!("ONE!");
	let x = Box::new(42);
	assert_eq!(*x, 42);

	log_debug!("TWO!");
	let y = Box::new(42);
	assert_eq!(*y, 42);

	log_debug!("THREE!");
	let mut vec = vec![1, 2, 3];
	assert_eq!(vec.len(), 3);
	log_debug!("FOUR!");

	vec.push(4);
	assert_eq!(vec.len(), 4);
	log_debug!("FIVE!");
}

#[test_case]
fn test_global_allocator_many_boxes() {
	for i in 0..1000 {
		let x = Box::new(i);
		assert_eq!(*x, i);
	}
}

#[test_case]
fn test_global_allocator_large_vec() {
	let mut vec = vec![0usize; 250];
	for (i, v) in vec.iter_mut().enumerate() {
		*v = i;
	}
	for (i, v) in vec.iter().enumerate() {
		assert_eq!(v, &i);
	}
}
