#![allow(non_snake_case)]

use std::ffi::c_void;
use crate::types::{DeviceId, DeviceInfo, PlatformId, PlatformInfo};

extern "C" {
    /// cl_int clGetPlatformIDs(cl_uint num_entries, cl_platform_id *platforms, cl_uint *num_platforms)
    pub fn clGetPlatformIDs(num_entries: u32, platforms: *mut PlatformId, num_platforms: *mut u32) -> i32;

    /// cl_int clGetPlatformInfo(cl_platform_id platform, cl_platform_info param_name, size_t param_value_size, void *param_value, size_t *param_value_size_ret)
    pub fn clGetPlatformInfo(platform: PlatformId, name: PlatformInfo, value_size: usize, value: *mut u8, value_size_ret: *mut usize) -> i32;

    /// cl_int clGetDeviceIDs(cl_platform_id platform, cl_device_type device_type, cl_uint num_entries, cl_device_id *devices, cl_uint *num_devices)
    pub fn clGetDeviceIDs(platform: PlatformId, device_type: u64, num_entries: u32, devices: *mut DeviceId, num_devices: *mut u32) -> i32;

    /// cl_int clGetDeviceInfo(cl_device_id device, cl_device_info param_name, size_t param_value_size, void *param_value, size_t *param_value_size_ret)
    pub fn clGetDeviceInfo(device: DeviceId, name: DeviceInfo, value_size: usize, value: *mut c_void, value_size_ret: *mut usize) -> i32;
}
