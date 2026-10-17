fn main() {
    // tauri-build embeds its Windows manifest (Common Controls v6) in the app binary only. Test
    // binaries link the same comctl32 imports and exit with STATUS_ENTRYPOINT_NOT_FOUND without it,
    // so link our copy of that manifest into every binary of this crate instead.
    let windows = tauri_build::WindowsAttributes::new_without_app_manifest();
    if let Err(e) = tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows)) {
        println!("cargo:warning={e:#}");
        std::process::exit(1);
    }

    let windows_msvc = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc");
    if windows_msvc {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("windows-app-manifest.xml");
        println!("cargo:rerun-if-changed={}", manifest.display());
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
    }
}
