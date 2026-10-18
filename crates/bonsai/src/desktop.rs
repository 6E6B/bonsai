//! Settings > Desktop: the tray icon, closing to the tray and starting on login.
//!
//! `sync` brings the tray and the login item in line with the settings; it runs at startup and
//! after any change to them. The login item is the OS's (a `.desktop` file, a registry value or
//! a launch agent), written by `tauri-plugin-autostart` on its own thread.

use std::sync::Arc;

use bonsai_core::Core;
use bonsai_core::settings::Settings;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Window, WindowEvent};
use tauri_plugin_autostart::ManagerExt;

/// Passed to the app when the OS starts it at login, so `start_hidden` only applies then.
pub const LOGIN_ARG: &str = "--login";
const TRAY: &str = "main";

/// The autostart plugin, set to pass [`LOGIN_ARG`].
pub fn plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        Some(vec![LOGIN_ARG]),
    )
}

/// At startup: the tray and login item as set, and the window shown unless started at login
/// with "Start in the tray". The window is created hidden (see `frame.rs`).
pub fn start(app: &AppHandle, s: &Settings) {
    sync(app, s);
    let at_login = std::env::args().any(|a| a == LOGIN_ARG);
    if !(at_login && s.tray && s.start_hidden)
        && let Some(w) = app.get_webview_window("main")
    {
        let _ = w.show();
    }
}

/// Shows or removes the tray icon and adds or removes the login item to match `s`.
pub fn sync(app: &AppHandle, s: &Settings) {
    let has_tray = app.tray_by_id(TRAY).is_some();
    if s.tray && !has_tray {
        if let Err(e) = add_tray(app) {
            tracing::warn!("tray icon: {e}");
        }
    } else if !s.tray && has_tray {
        drop(app.remove_tray_by_id(TRAY));
        // With no tray there is no way back to a hidden window.
        show(app);
    }
    let want = s.start_on_login;
    let app = app.clone();
    // The plugin writes files or the registry, which stays off the main thread.
    std::thread::spawn(move || {
        let launch = app.autolaunch();
        let on = launch.is_enabled().unwrap_or(!want);
        let r = match (want, on) {
            (true, false) => launch.enable(),
            (false, true) => launch.disable(),
            _ => Ok(()),
        };
        if let Err(e) = r
            && let Some(core) = app.try_state::<Arc<Core>>()
        {
            core.notice(format!("Couldn't change start on login: {e}"), true);
        }
    });
}

fn add_tray(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "show", "Show Bonsai", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &PredefinedMenuItem::separator(app)?, &quit])?;
    let mut tray = TrayIconBuilder::with_id(TRAY)
        .tooltip("Bonsai")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, e| match e.id().as_ref() {
            "show" => show(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, e| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = e
            {
                show(tray.app_handle());
            }
        });
    // A template lets macOS choose the menu bar color in light and dark mode.
    #[cfg(target_os = "macos")]
    {
        tray = tray
            .icon(tauri::include_image!("icons/tray-template.png"))
            .icon_as_template(true);
    }
    #[cfg(not(target_os = "macos"))]
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}

/// Brings the main window back from the tray or from behind other windows.
fn show(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// With "Close to tray", closing the main window hides it instead.
pub fn on_window_event(w: &Window, e: &WindowEvent) {
    let WindowEvent::CloseRequested { api, .. } = e else {
        return;
    };
    if w.label() != "main" {
        return;
    }
    let Some(core) = w.try_state::<Arc<Core>>() else {
        return;
    };
    let s = core.settings();
    if s.tray && s.close_to_tray {
        api.prevent_close();
        let _ = w.hide();
    }
}
