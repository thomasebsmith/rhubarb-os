use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicU64, AtomicBool};
use crate::thread::Thread;

pub struct SpinLock<T: ?Sized> {
    held: AtomicBool;      // Whether this lock is held
    holder_thread_id: u64; // Undefined value if !held
    value: UnsafeCell<T>;  // The value guarded by this lock
}

// Upon being dropped, releases the SpinLock that was used to acquire it.
pub struct MutexGuard<'a, T: ?Sized> {
    mutex: &'a SpinLock<T>;
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.mutex.value.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&mut self) -> &mut T {
        unsafe { self.mutex.value.get() }
    }
}

impl<T: ?Sized> SpinLock<T> {
    // Creates a new, unlocked SpinLock.
    pub fn new(t: T) -> Self {
        Self{AtomicBool::new(false), 0, UnsafeCell::new(t)}
    }

    // Locks this SpinLock, busy-waiting if needed.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        // Busy-wait until the lock is acquired.
        loop {
            match self.try_lock_weak() {
                Some(guard) => return guard,
                None        => continue,
            }
        }
    }

    // Attempts to locks this SpinLock but does not wait if this is not
    // possible. This may fail even if the lock is available.
    pub fn try_lock_weak(&self) -> Option<MutexGuard<'_, T>> {
        match self.held.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ) {
            Ok(_) => {
                self.holder_thread_id = Thread::get_current().id;
                Some(MutexGuard{&self})
            }
            Err(_) => None
        }
    }

    // Attempts to locks this SpinLock but does not wait if this is not
    // possible. This will succeed if the lock is available.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        match self.held.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ) {
            Ok(_) => {
                self.holder_thread_id = Thread::get_current().id;
                Some(MutexGuard{&self})
            }
            Err(_) => None
        }

    }

    fn unlock(&self) {
        self.held.store(false, Ordering::Release);
    }
}
