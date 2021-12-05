use std::sync::atomic::{AtomicU64, AtomicBool};

pub struct SpinLock<T: ?Sized> {
    AtomicU64 holder_thread_id;
    AtomicBool held;
}

impl<T> SpinLock<T> {
    pub fn new(t: T) -> SpinLock<T> {
        return SpinLock{0, 0, false};
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        // TODO: while !self.try_lock() {}
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        // TODO
    }
}
