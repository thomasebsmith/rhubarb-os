use std::sync::atomic::{AtomicU64, AtomicBool};

pub struct SpinLock<T: ?Sized> {
    AtomicU64 holder_thread_id; // Undefined value if !held
    AtomicBool held;            // Whether this lock is held
}

impl<T> SpinLock<T> {
    // Creates a new, unlocked SpinLock.
    pub fn new(t: T) -> SpinLock<T> {
        return SpinLock{0, 0, false};
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
        // TODO
    }
}
