use crate::types::{PlatformId, PlatformInfo};

extern "C" {
    /// cl_int clGetPlatformIDs(cl_uint num_entries, cl_platform_id *platforms, cl_uint *num_platforms)
    pub fn clGetPlatformIDs(num_entries: u32, platforms: *mut PlatformId, num_platforms: *mut u32) -> i32;

    /// cl_int clGetPlatformInfo(cl_platform_id platform, cl_platform_info param_name, size_t param_value_size, void *param_value, size_t *param_value_size_ret)
    pub fn clGetPlatformInfo(platform: PlatformId, name: PlatformInfo, value_size: usize, value: *mut u8, value_size_ret: *mut usize) -> i32;
}
