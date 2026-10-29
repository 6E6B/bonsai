//! Tauri shell. Owns [`Core`], exposes typed IPC commands (see [`ipc`]), serves thumbnails
//! over `thumb://` (see [`thumb`]), title bar icons over `wicon://` (see [`titlebar`]) and theme fonts over `font://` (see [`font`]), and tells the UI when the store changed (see [`notify`]).
//!
//! No logic lives here: commands read the store or forward to `bonsai-core`.

use std::sync::Arc;

use bonsai_core::{Core, CoreConfig};
use tauri::Manager;

mod asset;
mod bg;
mod captcha;
mod desktop;
mod font;
mod frame;
pub mod ipc;
mod notify;
mod schemes;
mod session_lock;
mod signin;
mod thumb;
mod titlebar;
mod updater;
mod vault;
mod wikiimg;

pub fn run() {
    notify::capture_local_offset();
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
        .plugin(desktop::plugin())
        .on_window_event(|w, e| {
            desktop::on_window_event(w, e);
            titlebar::on_window_event(w, e);
        })
        // Only the app's own window talks to the backend. The sign-in window shows
        // roblox.com and must never reach it (Tauri's ACL already denies remote pages).
        .invoke_handler(move |invoke| {
            if invoke.message.webview().label() != "main" {
                invoke.resolver.reject("not allowed from this window");
                return true;
            }
            if let Some(core) = invoke.message.webview().app_handle().try_state::<Arc<Core>>()
                && core.accounts_locked()
                && !matches!(
                    invoke.message.command(),
                    "vault_status"
                        | "vault_submit"
                        | "vault_activity"
                        | "window_frame"
                        | "window_buttons"
                        | "window_maximized"
                        | "run_command"
                )
            {
                invoke.resolver.reject("Unlock Bonsai to access accounts.");
                return true;
            }
            handler(invoke)
        })
        .register_asynchronous_uri_scheme_protocol(thumb::SCHEME, thumb::handle)
        .register_asynchronous_uri_scheme_protocol(asset::SCHEME, asset::handle)
        .register_asynchronous_uri_scheme_protocol(font::SCHEME, font::handle)
        .register_asynchronous_uri_scheme_protocol(bg::SCHEME, bg::handle)
        .register_asynchronous_uri_scheme_protocol(wikiimg::SCHEME, wikiimg::handle)
        .register_asynchronous_uri_scheme_protocol(titlebar::SCHEME, titlebar::handle)
        .setup(move |app| {
            builder.mount_events(app);
            let repaint = notify::spawn(app.handle().clone())?;
            let core = Arc::new(Core::start(CoreConfig::standard(), repaint));
            app.manage(frame::Frame::default());
            let titlebar = titlebar::Titlebar::read();
            // The window starts hidden so the frame can be set up before it is realized.
            if let Some(w) = app.get_webview_window("main") {
                if let Err(e) = schemes::allow(&w) {
                    tracing::warn!("could not allow the app's schemes: {e}");
                }
                if let Err(e) = frame::init(&w) {
                    tracing::warn!("could not set up the window frame: {e}");
                }
                titlebar.set_maximized(w.is_maximized().unwrap_or(false));
            }
            app.manage(titlebar);
            desktop::start(app.handle(), &core.settings());
            let handle = app.handle().clone();
            core.set_plugin_runner(Arc::new(move |account, id, arg| {
                let core = handle
                    .try_state::<Arc<Core>>()
                    .ok_or_else(|| "Not started.".to_owned())?;
                let inv = ipc::Invocation {
                    id: id.key().to_owned(),
                    arg,
                    as_account: Some(ipc::Id(account.0)),
                    context: None,
                };
                ipc::dispatch(&handle, &core, inv)
            }));
            app.manage(core);
            updater::start(app.handle())?;
            app.manage(vault::SystemLockWarning::default());
            vault::start_idle(app.handle().clone());
            vault::start_system_lock(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(e) = result {
        tracing::error!("app exited with an error: {e}");
    }
}
