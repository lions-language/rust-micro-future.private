use std::cell::UnsafeCell;

pub(crate) struct UnparkMutex<D> {
    status: AtomicUsize,
    inner: UnsafeCell<Option<D>>,
}

impl<D> UnparkMutex<D> {
    pub(crate) fn new() -> Self {
        Self {
            status: AtomicUsize::new(WAITING),
            inner: UnsafeCell::new(None)
        }
    }
}

