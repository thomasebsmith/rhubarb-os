pub struct SpinLock<T: ?Sized> {
    // TODO
}

impl<T> SpinLock<T> {
    pub fn new(t: T) -> SpinLock<T> {
        // TODO
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        // TODO
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        // TODO
    }
}
