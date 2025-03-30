use core::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicUsize, Ordering},
};

pub struct Mutex<T> {
	state: AtomicUsize,
	value: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T> {
	mutex: &'a Mutex<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
	pub const fn new(value: T) -> Self {
		Self {
			state: AtomicUsize::new(0),
			value: UnsafeCell::new(value),
		}
	}

	pub fn lock(&self) -> MutexGuard<T> {
		while self.state.swap(1, Ordering::Acquire) == 1 {}

		MutexGuard {
			mutex: self,
		}
	}
}

impl<T> Deref for MutexGuard<'_, T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe { &*self.mutex.value.get() }
	}
}

impl<T> DerefMut for MutexGuard<'_, T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe { &mut *self.mutex.value.get() }
	}
}

impl<T> Drop for MutexGuard<'_, T> {
	fn drop(&mut self) {
		self.mutex.state.store(0, Ordering::Release);
	}
}
