use std::fmt::{Display, Formatter};

/// Type representing an error code returned by OpenCL functions.
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[repr(i32)]
pub enum Error {
    /// An unsupported error code was received.
    Unknown = 1,

    /// The operation succeeded.
    Success = 0,
}

impl Default for Error {
    /// The default error code is [Error::Unknown].
    fn default() -> Self {
        Error::Unknown
    }
}

impl From<i32> for Error {
    fn from(code: i32) -> Self {
        match code {
            0 => Error::Success,
            _ => Error::Unknown,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Unknown => write!(f, "An unknown error occurred."),
            Error::Success => write!(f, "The operation completed successfully."),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_error_code() {
        assert_eq!(Error::default(), Error::Unknown);
    }
}
