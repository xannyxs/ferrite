use crate::sync::{mutex::MutexGuard, Mutex};

/// A wrapper type that provides synchronized access to an inner value using a
/// mutex. This type is particularly useful for creating thread-safe global
/// state, such as allocators.
pub struct Locked<A> {
	/// The protected inner value wrapped in a mutex
	inner: Mutex<A>,
}

impl<A> Locked<A> {
	/// Creates a new `Locked<A>` instance around a value.
	///
	/// # Arguments
	/// * `inner` - The value to be protected by the mutex
	///
	/// # Examples
	/// ```
	/// static PROTECTED: Locked<MyType> = Locked::new(MyType::new());
	/// ```
	pub const fn new(inner: A) -> Self {
		return Self {
			inner: Mutex::new(inner),
		};
	}

	/// Acquires the mutex and returns a guard that provides access to the inner
	/// value. The mutex will be automatically released when the guard is
	/// dropped.
	///
	/// # Returns
	/// * `MutexGuard<A>` - A RAII guard which will release the mutex when
	///   dropped
	///
	/// # Examples
	/// ```
	/// let guard = PROTECTED.lock();
	/// // Use the protected value
	/// // Guard automatically releases the mutex when it goes out of scope
	/// ```
	pub fn lock(&self) -> MutexGuard<A> {
		return self.inner.lock();
	}
}
