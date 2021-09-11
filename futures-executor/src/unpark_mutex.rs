use std::cell::UnsafeCell;

pub(crate) struct UnparkMutex<D> {
    status: AtomicUsize,
    inner: UnsafeCell<Option<D>>,
}

unsafe impl<D: Send> Send for UnparkMutex<D> {}
unsafe impl<D: Send> Sync for UnparkMutex<D> {}

const WAITING: usize = 0;
const POLLING: usize = 1;
const REPOLL: usize = 2;

impl<D> UnparkMutex<D> {
    pub(crate) fn new() -> Self {
        Self {
            status: AtomicUsize::new(WAITING),
            inner: UnsafeCell::new(None)
        }
    }

    pub(crate) fn notify(&self) -> Result<D, ()> {
        let mut status = self.status.load(SeqCst);
    }
}

