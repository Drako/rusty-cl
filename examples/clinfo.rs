use rusty_cl::device::cl_get_device_ids;
use rusty_cl::platform::Platform;
use rusty_cl::types::DeviceType;

fn main() {
    let platforms = Platform::get_all();
    println!("Number of platforms                               {}", platforms.len());
    for platform in &platforms {
        println!("  Platform Name                                   {}", platform.name());
        println!("  Platform Vendor                                 {}", platform.vendor());
        println!("  Platform Version                                {}", platform.version());
        println!("  Platform Profile                                {}", platform.profile());
        println!("  Platform Extensions                             {}", platform.extensions().join(" "));
    }
    if !platforms.is_empty() {
        println!();
    }

    for platform in &platforms {
        println!("  Platform Name                                   {}", platform.name());
        if let Ok(device_ids) = cl_get_device_ids(platform.id(), DeviceType::ALL) {
            println!("Number of devices                                 {}", device_ids.len());
        }
    }

    if let Ok(null_platform) = Platform::get(0) {
        println!();
        println!("NULL platform behavior");
        println!("  clGetPlatformInfo(NULL, CL_PLATFORM_NAME, ...)  {}", null_platform.name());
    }
}
