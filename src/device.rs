use std::ffi::c_void;
use crate::native::{clGetDeviceIDs, clGetDeviceInfo};
use crate::types::{DeviceId, DeviceInfo, DeviceType, PlatformId};
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

/// Get a device info for the given device.
///
/// # Safety
///
/// The caller must make sure the generic type parameter matches the type of the device info.
pub unsafe fn cl_get_device_info<T: Sized>(device: DeviceId, name: DeviceInfo) -> Result<T> {
    let mut value_size: usize = 0;
    let result = Error::from(clGetDeviceInfo(device, name, 0, std::ptr::null_mut(), &mut value_size));
    if result != Error::Success {
        return Err(result);
    }
    if value_size != std::mem::size_of::<T>() {
        return Err(Error::InvalidValue);
    }

    let mut value = std::mem::MaybeUninit::<T>::uninit();
    let result = Error::from(clGetDeviceInfo(device, name, value_size, value.as_mut_ptr() as *mut c_void, std::ptr::null_mut()));
    if result != Error::Success {
        return Err(result);
    }

    Ok(value.assume_init())
}

/// Get a device info for the given device.
///
/// # Safety
///
/// The caller must make sure the type of the device info is a actually a string.
pub unsafe fn cl_get_device_info_string(device: DeviceId, name: DeviceInfo) -> Result<String> {
    let mut value_size: usize = 0;
    let result = Error::from(clGetDeviceInfo(device, name, 0, std::ptr::null_mut(), &mut value_size));
    if result != Error::Success {
        return Err(result);
    }

    let mut value: Vec<u8> = vec![0; value_size];
    let result = Error::from(clGetDeviceInfo(device, name, value_size, value.as_mut_ptr() as *mut c_void, std::ptr::null_mut()));
    if result != Error::Success {
        return Err(result);
    }

    value.truncate(value.len() - 1);
    Ok(std::str::from_utf8_unchecked(value.as_slice()).to_string())
}
