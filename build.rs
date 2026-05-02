fn main() {
    // Declare expected cfg names so Cargo doesn't error when cfg_aliases emits them.
    // Note: `windows` and `unix` are built-in Rust cfg keys - no alias needed.
    // Required since Rust 1.80+ which validates --cfg flags from build scripts.
    println!("cargo:rustc-check-cfg=cfg(android)");
    println!("cargo:rustc-check-cfg=cfg(macos)");
    println!("cargo:rustc-check-cfg=cfg(ios)");
    println!("cargo:rustc-check-cfg=cfg(apple)");
    println!("cargo:rustc-check-cfg=cfg(linux)");

    cfg_aliases::cfg_aliases! {
        // Platforms
        // Note: `windows` is a built-in Rust cfg key (like `unix`), no alias needed.
        // #[cfg(windows)] works natively on Windows targets.
        android: { target_os = "android" },
        macos: { target_os = "macos" },
        ios: { target_os = "ios" },
        apple: { any(target_os = "ios", target_os = "macos") },
        linux: { all(unix, not(apple), not(android)) },
    }

    #[cfg(all(feature = "packager", target_os = "macos"))]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Resources/lib");

    // Embed Windows icon and version info into the .exe
    #[cfg(target_os = "windows")]
    {
        let rc_file = std::path::Path::new("resources/eesha.rc");
        if rc_file.exists() {
            embed_resource::compile(rc_file, embed_resource::NONE);
        }
    }
}
