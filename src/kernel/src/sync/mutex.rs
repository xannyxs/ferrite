use core::{
	cell::UnsafeCell,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicUsize, Ordering},
};

/// A simple spinlock-based mutual exclusion primitive.
///
/// This Mutex uses an atomic usize to track the lock state (0=unlocked,
/// 1=locked) and an `UnsafeCell` to allow interior mutability of the protected
/// data `T`.
pub struct Mutex<T> {
	state: AtomicUsize,
	value: UnsafeCell<T>,
}

/// An RAII implementation of a scoped lock for a `Mutex`.
///
/// When this structure is dropped (goes out of scope), the lock will be
/// automatically released. Provides access to the protected data via `Deref`
/// and `DerefMut`.
pub struct MutexGuard<'a, T> {
	mutex: &'a Mutex<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

#[allow(clippy::implicit_return)]
impl<T> Mutex<T> {
	/// Creates a new `Mutex` in an unlocked state containing the provided
	/// value.
	pub const fn new(value: T) -> Self {
		Self {
			state: AtomicUsize::new(0),
			value: UnsafeCell::new(value),
		}
	}

	/// Acquires the mutex lock, spinning until it becomes available.
	///
	/// This function will block the current thread (by spinning) until the lock
	/// can be acquired. Returns a `MutexGuard` which allows access to the
	/// protected data and automatically releases the lock when dropped.
	///
	/// # Panics
	/// This implementation does not handle potential deadlocks (e.g., trying
	/// to lock the same mutex twice on the same thread).
	pub fn lock(&self) -> MutexGuard<T> {
		while self.state.swap(1, Ordering::Acquire) == 1 {}

		MutexGuard {
			mutex: self,
		}
	}
}

#[allow(clippy::implicit_return)]
impl<T> Deref for MutexGuard<'_, T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe { &*self.mutex.value.get() }
	}
}

#[allow(clippy::implicit_return)]
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
