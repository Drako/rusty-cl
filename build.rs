#[cfg(target_os = "macos")]
fn link_macos_framework() {
    println!("cargo:rustc-link-search=framework=/System/Library/Frameworks");
    println!("cargo:rustc-link-lib=framework=OpenCL");
}

fn main() {
    #[cfg(target_os = "macos")]
    link_macos_framework();
}
