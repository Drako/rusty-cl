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
