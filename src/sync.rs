use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

type AcquireFn =
    fn(&AtomicBool, bool, bool, Ordering, Ordering) -> Result<bool, bool>;

/// A mutex that synchronizes using busy-waiting and atomic variables.
pub struct SpinLock<T: ?Sized> {
    held: AtomicBool,           // Whether this lock is held
    value: UnsafeCell<T>,       // The value guarded by this lock
}

/// Upon being dropped, releases the SpinLock that was used to acquire it.
pub struct MutexGuard<'a, T: ?Sized> {
    mutex: &'a SpinLock<T>,
}

// Lock acquisitions are thread-local, so MutexGuards cannot be transferred
// among threads.
impl<T: ?Sized> !Send for MutexGuard<'_, T> {}

// Lock acquisitions can be accessed from multiple threads. Rust's reference
// rules enforce that references are safely used.
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    /// On drop, unlocks the mutex.
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    /// Gets the value guarded by the mutex.
    ///
    /// # Return value:
    /// A reference to the value
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    /// Gets the value guarded by the mutex.
    ///
    /// # Return value:
    /// A mutable reference to the value
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> SpinLock<T> {
    /// Creates a SpinLock.
    ///
    /// # Arguments:
    ///
    /// * `t` - The value that the new SpinLock should hold.
    ///
    /// # Return value:
    /// A new, unlocked SpinLock, holding `t`.
    pub const fn new(t: T) -> Self {
        Self { held: AtomicBool::new(false), value: UnsafeCell::new(t) }
    }
}

impl<T: ?Sized> SpinLock<T> {
    /// Locks this SpinLock, busy-waiting if needed.
    ///
    /// # Return value:
    /// A struct that provides access to the guarded value and unlocks this
    /// mutex when it is dropped
    pub fn lock(&self) -> MutexGuard<'_, T> {
        // Busy-wait until the lock is acquired.
        loop {
            match self.try_lock_weak() {
                Some(guard) => return guard,
                None        => continue,
            }
        }
    }

    fn try_lock_helper(
        &self,
        acquire_fn: AcquireFn
    ) -> Option<MutexGuard<'_, T>> {
        match acquire_fn(
            &self.held,
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ) {
            Ok(_) => Some(MutexGuard { mutex: &self }),
            Err(_) => None
        }

    }

    /// Attempts to locks this SpinLock but does not wait if this is not
    /// possible. This may fail even if the lock is available.
    ///
    /// # Return value:
    /// None if locking failed, or a guard for this mutex if locking succeeded.
    pub fn try_lock_weak(&self) -> Option<MutexGuard<'_, T>> {
        self.try_lock_helper(AtomicBool::compare_exchange_weak)
    }

    /// Attempts to locks this SpinLock but does not wait if this is not
    /// possible. This will succeed if the lock is available.
    ///
    /// # Return value:
    /// None if locking failed, or a guard for this mutex if locking succeeded.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.try_lock_helper(AtomicBool::compare_exchange)
    }

    // Unlocks this SpinLock. No busy-waiting is needed. This should not be
    // called manually.
    fn unlock(&self) {
        self.held.store(false, Ordering::Release);
    }
}

// SpinLocks can be sent to other threads or accessed concurrently.
unsafe impl<T: ?Sized + Send> Send for SpinLock<T> {}
unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}
