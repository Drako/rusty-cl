use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitOr, Not};
use std::str::FromStr;

/// cl_platform_id
pub type PlatformId = usize;

/// cl_device_id
pub type DeviceId = usize;

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

/// cl_device_info
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[repr(u32)]
#[allow(missing_docs)]
pub enum DeviceInfo {
    Type = 0x1000,
    VendorId = 0x1001,
    MaxComputeUnits = 0x1002,
    MaxWorkItemDimensions = 0x1003,
    MaxWorkGroupSize = 0x1004,
    MaxWorkItemSizes = 0x1005,
    PreferredVectorWidthChar = 0x1006,
    PreferredVectorWidthShort = 0x1007,
    PreferredVectorWidthInt = 0x1008,
    PreferredVectorWidthLong = 0x1009,
    PreferredVectorWidthFloat = 0x100A,
    PreferredVectorWidthDouble = 0x100B,
    MaxClockFrequency = 0x100C,

    /// The default compute device address space size specified as an unsigned integer value in bits. Currently supported values are 32 or 64 bits.
    AddressBits = 0x100D,
    MaxReadImageArgs = 0x100E,
    MaxWriteImageArgs = 0x100F,
    MaxMemAllocSize = 0x1010,
    Image2dMaxWidth = 0x1011,
    Image2dMaxHeight = 0x1012,
    Image3dMaxWidth = 0x1013,
    Image3dMaxHeight = 0x1014,
    Image3dMaxDepth = 0x1015,
    ImageSupport = 0x1016,
    MaxParameterSize = 0x1017,
    MaxSamplers = 0x1018,
    MemBaseAddrAlign = 0x1019,
    MinDataTypeAlignSize = 0x101A,
    SingleFpConfig = 0x101B,

    /// Type of global memory cache supported.
    ///
    /// Valid values are: `CL_NONE`, `CL_READ_ONLY_CACHE`, and `CL_READ_WRITE_CACHE`.
    GlobalMemCacheType = 0x101C,

    /// Size of global memory cache line in bytes.
    GlobalMemCachelineSize = 0x101D,

    /// Size of global memory cache in bytes.
    GlobalMemCacheSize = 0x101E,

    /// Size of global device memory in bytes.
    GlobalMemSize = 0x101F,
    MaxConstantBufferSize = 0x1020,
    MaxConstantArgs = 0x1021,
    LocalMemType = 0x1022,
    LocalMemSize = 0x1023,

    /// Is `true` if the device implements error correction for the memories, caches, registers etc. in the device.
    /// Is `false` if the device does not implement error correction. This can be a requirement for certain clients of OpenCL.
    ErrorCorrectionSupport = 0x1024,
    ProfilingTimerResolution = 0x1025,

    /// Is `true` if the OpenCL device is a little endian device and `false` otherwise.
    EndianLittle = 0x1026,

    /// Is `true` if the device is available and `false` if the device is not available.
    Available = 0x1027,

    /// Is `false` if the implementation does not have a compiler available to compile the program source.
    /// Is `true` if the compiler is available. This can be CL_FALSE for the embededed platform profile only.
    CompilerAvailable = 0x1028,

    /// Describes the execution capabilities of the device.
    ///
    /// This is a bit-field that describes one or more of the following values:
    ///  * `CL_EXEC_KERNEL` - The OpenCL device can execute OpenCL kernels.
    ///  * `CL_EXEC_NATIVE_KERNEL` - The OpenCL device can execute native kernels.
    ///
    /// The mandated minimum capability is `CL_EXEC_KERNEL`.
    ExecutionCapabilities = 0x1029,

    #[deprecated]
    #[allow(missing_docs)]
    QueueProperties = 0x102A,

    /// Device name string.
    Name = 0x102B,
    Vendor = 0x102C,
    DriverVersion = 0x102D,
    Profile = 0x102E,
    Version = 0x102F,

    /// Returns a space separated list of extension names (the extension names themselves do not contain any spaces).
    Extensions = 0x1030,

    /// The platform associated with this device.
    Platform = 0x1031,

    /// Describes the OPTIONAL double precision floating-point capability of the OpenCL device.
    ///
    /// This is a bit-field that describes one or more of the following values:
    ///  * `CL_FP_DENORM` - denorms are supported.
    ///  * `CL_FP_INF_NAN` - INF and NaNs are supported.
    ///  * `CL_FP_ROUND_TO_NEAREST` - round to nearest even rounding mode supported.
    ///  * `CL_FP_ROUND_TO_ZERO` - round to zero rounding mode supported.
    ///  * `CL_FP_ROUND_TO_INF` - round to +ve and -ve infinity rounding modes supported.
    ///  * `CL_FP_FMA` - IEEE754-2008 fused multiply-add is supported.
    ///
    /// The mandated minimum double precision floating-point capability is
    /// `CL_FP_FMA | CL_FP_ROUND_TO_NEAREST | CL_FP_ROUND_TO_ZERO | CL_FP_ROUND_TO_INF | CL_FP_INF_NAN | CL_FP_DENORM`.
    DoubleFpConfig = 0x1032,

    PreferredVectorWidthHalf = 0x1034,

    #[deprecated]
    #[allow(missing_docs)]
    HostUnifiedMemory = 0x1035,

    NativeVectorWidthChar = 0x1036,
    NativeVectorWidthShort = 0x1037,
    NativeVectorWidthInt = 0x1038,
    NativeVectorWidthLong = 0x1039,
    NativeVectorWidthFloat = 0x103A,
    NativeVectorWidthDouble = 0x103B,
    NativeVectorWidthHalf = 0x103C,
    OpenclCVERSION = 0x103D,
    LinkerAvailable = 0x103E,
    BuiltInKernels = 0x103F,
    ImageMaxBufferSize = 0x1040,
    ImageMaxArraySize = 0x1041,
    ParentDevice = 0x1042,
    PartitionMaxSubDevices = 0x1043,
    PartitionProperties = 0x1044,
    PartitionAffinityDomain = 0x1045,
    PartitionType = 0x1046,
    ReferenceCount = 0x1047,
    PreferredInteropUserSync = 0x1048,
    PrintfBufferSize = 0x1049,
}

/// Device type to query/filter for or type of a given device.
#[derive(Eq, PartialEq, Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
#[repr(transparent)]
pub struct DeviceType(u64);

impl DeviceType {
    /// The default device of the given platform.
    pub const DEFAULT: Self = DeviceType(1 << 0);
    /// A CPU.
    pub const CPU: Self = DeviceType(1 << 1);
    /// A GPGPU.
    pub const GPU: Self = DeviceType(1 << 2);
    /// A dedicated compute card.
    pub const ACCELERATOR: Self = DeviceType(1 << 3);
    /// A custom device.
    pub const CUSTOM: Self = DeviceType(1 << 4);
    /// All device types.
    pub const ALL: Self = DeviceType(0xFFFFFFFF);

    /// Get the raw underlying value.
    pub fn raw(&self) -> u64 {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl BitOr for DeviceType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for DeviceType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Not for DeviceType {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self((!self.0) & DeviceType::ALL.0)
    }
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
