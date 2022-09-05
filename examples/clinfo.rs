use rusty_cl::platform::Platform;

fn main() {
    let platforms = Platform::get_all();
    println!("Number of platforms                               {}", platforms.len());
    for platform in platforms {
        println!("  Platform Name                                   {}", platform.name());
        println!("  Platform Vendor                                 {}", platform.vendor());
        println!("  Platform Version                                {}", platform.version());
        println!("  Platform Profile                                {}", platform.profile());
        println!("  Platform Extensions                             {}", platform.extensions().join(" "));
    }

    if let Ok(null_platform) = Platform::get(0) {
        println!();
        println!("NULL platform behavior");
        println!("  clGetPlatformInfo(NULL, CL_PLATFORM_NAME, ...)  {}", null_platform.name());
    }
}
