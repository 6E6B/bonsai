//! Tauri shell. Owns [`Core`], exposes typed IPC commands (see [`ipc`]), serves thumbnails
//! over `thumb://` (see [`thumb`]) and theme fonts over `font://` (see [`font`]), and tells the UI when the store changed (see [`notify`]).
//!
//! No logic lives here: commands read the store or forward to `rbx-core`.

use std::sync::Arc;

use rbx_core::{Core, CoreConfig};
use tauri::Manager;

mod captcha;
mod font;
pub mod ipc;
mod notify;
mod signin;
mod thumb;

pub fn run() {
    let t0 = std::time::Instant::now(); // PROFILE-TMP
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let builder = ipc::builder();
    #[cfg(debug_assertions)]
    if let Err(e) = ipc::export(&builder) {
        tracing::warn!("could not export TypeScript bindings: {e}");
    }

    let handler = builder.invoke_handler();
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        // Only the app's own window talks to the backend. The sign-in window shows
        // roblox.com and must never reach it (Tauri's ACL already denies remote pages).
        .invoke_handler(move |invoke| {
            if invoke.message.webview().label() != "main" {
                invoke.resolver.reject("not allowed from this window");
                return true;
            }
            handler(invoke)
        })
        .register_asynchronous_uri_scheme_protocol(thumb::SCHEME, thumb::handle)
        .register_asynchronous_uri_scheme_protocol(font::SCHEME, font::handle)
        .on_page_load(move |w, p| eprintln!("PROFILE {:?} page {:?} {}", t0.elapsed(), p.event(), w.label())) // PROFILE-TMP
        .setup(move |app| {
            eprintln!("PROFILE {:?} setup start", t0.elapsed()); // PROFILE-TMP
            builder.mount_events(app);
            let repaint = notify::spawn(app.handle().clone())?;
            app.manage(Arc::new(Core::start(CoreConfig::standard(), repaint)));
            eprintln!("PROFILE {:?} setup end", t0.elapsed()); // PROFILE-TMP
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(e) = result {
        tracing::error!("app exited with an error: {e}");
    }
}
