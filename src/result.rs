use std::fmt::{Display, Formatter};

/// Type representing an error code returned by OpenCL functions.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(i32)]
pub enum Error {
    /// An unsupported error code was received.
    Unknown = 1,

    /// The operation succeeded.
    Success = 0,

    /// An invalid value was passed as parameter.
    InvalidValue = -30,

    /// An invalid platform ID was passed as parameter.
    InvalidPlatform = -32,
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
            -30 => Error::InvalidValue,
            -32 => Error::InvalidPlatform,
            _ => Error::Unknown,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Unknown => write!(f, "An unknown error occurred."),
            Error::Success => write!(f, "The operation completed successfully."),
            Error::InvalidValue => write!(f, "An invalid value was passed as parameter."),
            Error::InvalidPlatform => write!(f, "An invalid platform ID was passed as parameter."),
        }
    }
}

/// The result type for OpenCL operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_error_code() {
        assert_eq!(Error::default(), Error::Unknown);
    }
}
