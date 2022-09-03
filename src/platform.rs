use crate::native::clGetPlatformIDs;
use crate::types::PlatformId;

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
