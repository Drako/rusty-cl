use crate::native::{clGetPlatformIDs, clGetPlatformInfo};
use crate::result::{Error, Result};
use crate::types::{PlatformId, PlatformInfo, Profile};

/// Get all available platform IDs.
///
/// # Remarks
///
/// While the underlying function may fail, when invalid parameters are supplied,
/// this function always succeeds, as make sure not to pass such invalid parameters.
pub fn cl_get_platform_ids() -> Vec<PlatformId> {
    let mut num_platforms: u32 = 0;
    let result = unsafe { clGetPlatformIDs(0, std::ptr::null_mut(), &mut num_platforms) };
    debug_assert_eq!(result, 0);

    let mut platform_ids: Vec<PlatformId> = vec![0; num_platforms as usize];
    if num_platforms == 0 {
        return platform_ids;
    }

    let result = unsafe { clGetPlatformIDs(num_platforms, platform_ids.as_mut_ptr(), std::ptr::null_mut()) };
    debug_assert_eq!(result, 0);

    platform_ids
}

/// Get a platform info for the given platform.
///
/// # Arguments
///
/// * `platform` - The platform for which the info should be queried.
/// * `name` - Which info to query.
///
/// # Errors
///
/// The following errors may be returned:
///
/// * `Error::InvalidPlatform` - An invalid platform ID was passed.
///
/// # Examples
///
/// ```no_run
/// # use rusty_cl::platform::{cl_get_platform_ids, cl_get_platform_info};
/// # use rusty_cl::types::PlatformInfo;
/// # fn main() -> rusty_cl::result::Result<()> {
/// for platform_id in cl_get_platform_ids() {
///     println!("Profile: {}", cl_get_platform_info(platform_id, PlatformInfo::Profile)?);
///     println!("Version: {}", cl_get_platform_info(platform_id, PlatformInfo::Version)?);
///     println!("Name: {}", cl_get_platform_info(platform_id, PlatformInfo::Name)?);
///     println!("Vendor: {}", cl_get_platform_info(platform_id, PlatformInfo::Vendor)?);
///     println!("Extensions: {}", cl_get_platform_info(platform_id, PlatformInfo::Extensions)?);
/// }
/// # Ok(())
/// # }
/// ```
pub fn cl_get_platform_info(platform: PlatformId, name: PlatformInfo) -> Result<String> {
    let mut len: usize = 0;
    let result = unsafe { clGetPlatformInfo(platform, name, 0, std::ptr::null_mut(), &mut len) };
    if result != 0 {
        return Err(Error::from(result));
    }

    if len == 0 {
        return Ok(String::new());
    }

    let mut content: Vec<u8> = vec![0; len];
    let result = unsafe { clGetPlatformInfo(platform, name, len, content.as_mut_ptr(), std::ptr::null_mut()) };
    if result != 0 {
        return Err(Error::from(result));
    }

    content.truncate(content.len() - 1);
    Ok(unsafe { std::str::from_utf8_unchecked(content.as_slice()) }.to_string())
}

/// Structure containing information about an OpenCL platform.
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Platform {
    id: PlatformId,
    profile: Profile,
    version: String,
    name: String,
    vendor: String,
    extensions: Vec<String>,
}

impl Platform {
    /// Get the platform information for the given platform ID.
    ///
    /// # Errors
    ///
    /// The following errors may be returned:
    ///
    /// * `Error::InvalidPlatform` - An invalid platform ID was passed.
    pub fn get(id: PlatformId) -> Result<Self> {
        Ok(Self {
            id,
            profile: Profile::try_from(cl_get_platform_info(id, PlatformInfo::Profile)?).unwrap(),
            version: cl_get_platform_info(id, PlatformInfo::Version)?,
            name: cl_get_platform_info(id, PlatformInfo::Name)?,
            vendor: cl_get_platform_info(id, PlatformInfo::Vendor)?,
            extensions: cl_get_platform_info(id, PlatformInfo::Extensions)?.split(" ").map(String::from).collect(),
        })
    }

    /// Get all available platforms.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rusty_cl::platform::Platform;
    /// for platform in Platform::get_all() {
    ///     println!("Profile: {}", platform.profile());
    ///     println!("Version: {}", platform.version());
    ///     println!("Name: {}", platform.name());
    ///     println!("Vendor: {}", platform.vendor());
    ///     println!("Extensions: {}", platform.extensions().join(" "));
    /// }
    /// ```
    pub fn get_all() -> Vec<Platform> {
        cl_get_platform_ids().into_iter().filter_map(|id| Platform::get(id).ok()).collect()
    }

    /// The ID of the platform.
    pub fn id(&self) -> PlatformId {
        self.id
    }

    /// The profile of the platform.
    pub fn profile(&self) -> Profile {
        self.profile
    }

    /// The OpenCL version of the platform.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// The name of the platform.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The vendor of the platform.
    pub fn vendor(&self) -> &str {
        &self.vendor
    }

    /// The available extensions on the platform.
    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}

impl Default for Platform {
    /// Get the default platform of the system.
    ///
    /// # Panics
    ///
    /// This function may panic if there are no available platforms.
    fn default() -> Self {
        Platform::get(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_platforms() {
        let platform_ids = cl_get_platform_ids();
        for id in platform_ids {
            assert_ne!(id, 0);
            let platform = Platform::get(id).unwrap();
            assert_eq!(id, platform.id());
            let profile: Profile = cl_get_platform_info(id, PlatformInfo::Profile).unwrap().try_into().unwrap();
            assert_eq!(profile, platform.profile());
            assert_eq!(cl_get_platform_info(id, PlatformInfo::Version).unwrap(), platform.version());
            assert_eq!(cl_get_platform_info(id, PlatformInfo::Name).unwrap(), platform.name());
            assert_eq!(cl_get_platform_info(id, PlatformInfo::Vendor).unwrap(), platform.vendor());
            assert_eq!(cl_get_platform_info(id, PlatformInfo::Extensions).unwrap(), platform.extensions().join(" "));
        }
    }
}
