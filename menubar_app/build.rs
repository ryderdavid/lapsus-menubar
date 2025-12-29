fn main() {
    // Set up build metadata for macOS
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
    }
    
    // This build script can be extended to:
    // - Copy icon resources to the build output
    // - Generate Info.plist for app bundle
    // - Set up code signing (if needed)
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=icons/");
}
