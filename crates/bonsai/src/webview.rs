//! WebKitGTK settings that keep the webview's memory down.
//!
//! The default cache model, `WebBrowser`, keeps large in-memory caches of pages and resources
//! for back/forward navigation across sites. Bonsai is one local page whose images come from
//! `thumb://`, which `bonsai-core` already caches, so the smaller `DocumentBrowser` model is
//! enough. `DocumentViewer` turns the memory cache off entirely, which makes thumbnails flash
//! their fallback when a virtual list mounts a row again.

use tauri::WebviewWindow;

#[cfg(target_os = "linux")]
pub fn tune(w: &WebviewWindow) -> Result<(), String> {
    use webkit2gtk::{CacheModel, WebContextExt, WebViewExt};

    w.with_webview(|v| {
        if let Some(ctx) = v.inner().context() {
            ctx.set_cache_model(CacheModel::DocumentBrowser);
        }
    })
    .map_err(|e| e.to_string())
}

#[cfg(not(target_os = "linux"))]
pub fn tune(_: &WebviewWindow) -> Result<(), String> {
    Ok(())
}
