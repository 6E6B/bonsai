// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // WebKitGTK on Wayland with the NVIDIA driver dies with "Error 71 (Protocol error)"
    // unless explicit sync is off. Other drivers ignore the variable. A user setting wins.
    #[cfg(target_os = "linux")]
    if std::env::var_os("__NV_DISABLE_EXPLICIT_SYNC").is_none() {
        // SAFETY: first thing in main, before any other thread exists.
        unsafe { std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1") };
    }
    bonsai_tauri::run();
}
