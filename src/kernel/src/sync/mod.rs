//! Synchronization primitives for the kernel.
//!
//! This module provides basic tools for ensuring safe access to shared data
//! in concurrent contexts, such as mutexes and wrappers for synchronized data.

/// Module containing the `Locked<T>` wrapper type for mutex-protected data.
pub mod locked;
/// Module containing the spinlock-based `Mutex<T>` implementation.
pub mod mutex;

pub use locked::Locked;
pub use mutex::Mutex;
