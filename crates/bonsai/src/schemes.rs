//! WebKitGTK 2.54 refuses to show `thumb:`, `bg:` and the other custom-scheme URLs on an
//! `http:` page ("Unsafe attempt to load URL ... Domains, protocols and ports must match"), so
//! under `tauri dev`, where the page comes from the Vite server, no image loads. Built apps load
//! from `tauri://localhost` and are not affected. Putting the schemes on the webview's CORS
//! allowlist lets the page load them again.

use tauri::WebviewWindow;

#[cfg(target_os = "linux")]
pub fn allow(w: &WebviewWindow) -> Result<(), String> {
    use webkit2gtk::WebViewExt;

    w.with_webview(|v| {
        let list: Vec<String> = [
            crate::thumb::SCHEME,
            crate::bg::SCHEME,
            crate::asset::SCHEME,
            crate::font::SCHEME,
        ]
        .iter()
        .map(|s| format!("{s}://*/*"))
        .collect();
        let list: Vec<&str> = list.iter().map(String::as_str).collect();
        v.inner().set_cors_allowlist(&list);
    })
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "linux"))]
pub fn allow(_: &WebviewWindow) -> Result<(), String> {
    Ok(())
}
