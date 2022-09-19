use crate::native::clGetDeviceIDs;
use crate::types::{DeviceId, DeviceType, PlatformId};
use crate::result::{Result, Error};

/// Get all available device IDs with the given device type on the given platform.
///
/// # Arguments
///
/// * `platform` - The platform for which the devices should be queried.
/// * `device_type` - The types of devices to query.
///
/// # Errors
///
/// The following errors may be returned:
///
/// * `Error::InvalidPlatform` - An invalid platform ID was passed.
/// * `Error::InvalidDeviceType` - An invalid device type was passed.
pub fn cl_get_device_ids(platform: PlatformId, device_type: DeviceType) -> Result<Vec<DeviceId>> {
    let mut num_devices: u32 = 0;
    let result = Error::from(unsafe { clGetDeviceIDs(platform, device_type.raw(), 0, std::ptr::null_mut(), &mut num_devices) });
    match result {
        Error::DeviceNotFound => return Ok(vec![]),
        Error::Success => {}
        _ => return Err(result),
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