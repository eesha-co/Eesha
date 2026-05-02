fn main() {
    // Declare expected cfg names so Cargo doesn't error when cfg_aliases emits them.
    // Required since Rust 1.80+ which validates --cfg flags from build scripts.
    println!("cargo:rustc-check-cfg=cfg(android)");
    println!("cargo:rustc-check-cfg=cfg(macos)");
    println!("cargo:rustc-check-cfg=cfg(ios)");
    println!("cargo:rustc-check-cfg=cfg(windows)");
    println!("cargo:rustc-check-cfg=cfg(apple)");
    println!("cargo:rustc-check-cfg=cfg(linux)");

    cfg_aliases::cfg_aliases! {
        // Platforms
        android: { target_os = "android" },
        macos: { target_os = "macos" },
        ios: { target_os = "ios" },
        windows: { target_os = "windows" },
        apple: { any(target_os = "ios", target_os = "macos") },
        linux: { all(unix, not(apple), not(android)) },
    }

    #[cfg(all(feature = "packager", target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Resources/lib");
}
