use crate::native::{clGetPlatformIDs, clGetPlatformInfo};
use crate::types::{PlatformId, PlatformInfo};
use crate::result::{Error, Result};

/// Get all available platform IDs.
///
/// # REMARKS
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
/// # Examples
///
/// ```no_run
/// # use rusty_cl::platform::{cl_get_platform_ids, cl_get_platform_info};
/// # use rusty_cl::types::PlatformInfo;
/// for platform_id in cl_get_platform_ids() {
///     println!("Profile: {}", cl_get_platform_info(platform_id, PlatformInfo::Profile)?);
///     println!("Version: {}", cl_get_platform_info(platform_id, PlatformInfo::Version)?);
///     println!("Name: {}", cl_get_platform_info(platform_id, PlatformInfo::Name)?);
///     println!("Vendor: {}", cl_get_platform_info(platform_id, PlatformInfo::Vendor)?);
///     println!("Extensions: {}", cl_get_platform_info(platform_id, PlatformInfo::Extensions)?);
/// }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_platform_ids() {
        let platform_ids = cl_get_platform_ids();
        for id in platform_ids {
            assert_ne!(id, 0);
        }
    }
}
