use std::fmt::{Display, Formatter};

/// cl_platform_id
pub type PlatformId = usize;

/// cl_platform_info
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[repr(u32)]
pub enum PlatformInfo {
    /// Either `"FULL_PROFILE"` or `"EMBEDDED_PROFILE"`.
    Profile = 0x0900,
    /// OpenCL version string.
    Version = 0x0901,
    /// Platform name string.
    Name = 0x0902,
    /// Platform vendor string.
    Vendor = 0x0903,
    /// Space-separated list of extension names (the extension names themselves do not contain any spaces) supported by the platform.
    Extensions = 0x0904,
}

/// An OpenCL profile (Full or embedded).
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
pub enum Profile {
    /// The implementation supports the OpenCL specification.
    Full,

    /// The implementation supports the OpenCL embedded profile.
    /// The embedded profile is defined to be a subset for each version of OpenCL.
    Embedded,
}

impl TryFrom<String> for Profile {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FULL_PROFILE" => Ok(Profile::Full),
            "EMBEDDED_PROFILE" => Ok(Profile::Embedded),
            _ => Err(value),
        }
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Profile::Full => write!(f, "FULL_PROFILE"),
            Profile::Embedded => write!(f, "EMBEDDED_PROFILE"),
        }
    }
}
