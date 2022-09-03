use crate::types::PlatformId;

extern "C" {
    /// cl_int clGetPlatformIDs(cl_uint num_entries, cl_platform_id *platforms, cl_uint *num_platforms)
    pub fn clGetPlatformIDs(num_entries: u32, platforms: *mut PlatformId, num_platforms: *mut u32) -> i32;
}
