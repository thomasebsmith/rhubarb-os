use std::sync::atomic::{AtomicU64, AtomicBool};
use crate::thread::Thread;

pub struct SpinLock<T: ?Sized> {
    held: AtomicBool;      // Whether this lock is held
    holder_thread_id: u64; // Undefined value if !held
    value: T;              // The value guarded by this lock
}

// Upon being dropped, releases the SpinLock that was used to acquire it.
pub struct MutexGuard<'a, T> {
    mutex: &'a SpinLock<T>;
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}

impl<T> SpinLock<T> {
    // Creates a new, unlocked SpinLock.
    pub fn new(t: T) -> SpinLock<T> {
        return SpinLock{AtomicBool::new(false), 0, t};
    }

    // Locks this SpinLock, busy-waiting if needed.
    pub fn lock(&self) -> MutexGuard<'_, T> {
        // Busy-wait until the lock is acquired.
        loop {
            match self.try_lock() {
                Some(guard) => return guard,
                None        => continue,
            }
        }
    }

    // Attempts to locks this SpinLock but does not wait if this
    // is not possible.
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        // TODO: Check this memory order.
        match self.held.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Acquire
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
