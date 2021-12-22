use std::sync::atomic::{AtomicU64, AtomicBool};

pub struct SpinLock<T: ?Sized> {
    holder_thread_id: AtomicU64; // Undefined value if !held
    held: AtomicBool;            // Whether this lock is held
    value: T;                    // The value guarded by this lock
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
        return SpinLock{AtomicU64::new(0), AtomicBool::new(false), t};
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
            Ok(_) => Some(MutexGuard{&self}) // TODO: Set holder_thread_id.
            Err(_) => None
        }
    }

    fn unlock(&self) {
        // TODO
    }
}
