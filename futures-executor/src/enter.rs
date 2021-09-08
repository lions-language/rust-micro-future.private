use std::fmt;

pub struct Enter {
    _priv: (),
}

pub struct EnterError {
    _priv: (),
}

impl fmt::Debug for EnterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // => EnterError { }
        f.debug_struct("EnterError").finish()
    }
}

impl fmt::Display for EnterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "an execution scope has already been entered")
    }
}

impl std::error::Error for EnterError {}
