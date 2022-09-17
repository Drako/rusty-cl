use crate::native::clGetDeviceIDs;
use crate::types::{DeviceId, DeviceType, PlatformId};
use crate::result::{Result, Error};

/// Get all available device IDs with the given device type on the given platform.
pub fn cl_get_device_ids(platform: PlatformId, device_type: DeviceType) -> Result<Vec<DeviceId>> {
    let mut num_devices: u32 = 0;
    let result = unsafe { clGetDeviceIDs(platform, device_type.raw(), 0, std::ptr::null_mut(), &mut num_devices) };
    if result != 0 {
        return Err(Error::from(result));
    }

    let mut device_ids: Vec<DeviceId> = vec![0; num_devices as usize];
    if num_devices == 0 {
        return Ok(device_ids);
    }

    let result = unsafe { clGetDeviceIDs(platform, device_type.raw(), num_devices, device_ids.as_mut_ptr(), std::ptr::null_mut()) };
    if result != 0 {
        return Err(Error::from(result));
    }

    Ok(device_ids)
}