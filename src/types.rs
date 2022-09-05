use std::fmt::{Display, Formatter};
use std::str::FromStr;

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

/// OpenCL Version information.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Version {
    /// The OpenCL major version.
    major: u8,

    /// The OpenCL minor version.
    minor: u8,

    /// Platform specific additional version information.
    extra: Option<String>,
}

impl Version {
    /// The OpenCL major version.
    pub fn major(&self) -> u8 {
        self.major
    }

    /// The OpenCL minor version.
    pub fn minor(&self) -> u8 {
        self.minor
    }

    /// Platform specific additional version information.
    pub fn extra(&self) -> &Option<String> {
        &self.extra
    }
}

impl TryFrom<String> for Version {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.splitn(3, " ").collect();
        if !(2..=3).contains(&parts.len()) || parts[0] != "OpenCL" {
            return Err(value);
        }

        let (major_str, minor_str) = parts[1].split_once('.').ok_or_else(|| value.clone())?;
        let major = u8::from_str(major_str).or_else(|_| Err(value.clone()))?;
        let minor = u8::from_str(minor_str).or_else(|_| Err(value.clone()))?;

        let extra = if parts.len() == 3 {
            Some(parts[2].to_string())
        } else {
            None
        };

        Ok(Version { major, minor, extra })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.extra() {
            Some(extra) => write!(f, "OpenCL {}.{} {}", self.major, self.minor, extra),
            None => write!(f, "OpenCL {}.{}", self.major, self.minor),
        }
    }
}
