//! The main window's frame on Linux. The window has no native title bar on any platform (the UI
//! draws its own), but on Linux GTK can still draw its client-side frame around the webview: the
//! theme's shadow and the resize handles outside the window. Tao already gives an undecorated
//! Wayland window an empty GTK title bar, which is what makes GTK draw the frame itself; turning
//! decorations back on makes the frame visible. X11 keeps the bare window.
//!
//! GTK3 does not clip the webview to the frame's rounded corners, so the corners are drawn by the
//! page: the window and webview backgrounds are transparent, GTK rounds the shadow, and the UI
//! clips itself to the same radius ([`WindowFrame`]). Maximized, tiled and fullscreen windows are
//! square.

use std::sync::atomic::{AtomicU32, Ordering};

use tauri::WebviewWindow;

use crate::ipc::WindowFrame;

/// Corner radius of the rounded frame, in CSS px.
#[cfg(target_os = "linux")]
const RADIUS: u32 = 8;

/// The corner radius the UI should clip to now. 0 means square.
#[derive(Default)]
pub struct Frame(AtomicU32);

impl Frame {
    pub fn get(&self) -> WindowFrame {
        WindowFrame {
            radius: self.0.load(Ordering::Relaxed),
        }
    }
}

/// Turns the frame on. Call once, on the main thread, while the window is still hidden: the
/// shadow needs an RGBA visual, which GTK only takes before the window is realized.
#[cfg(target_os = "linux")]
pub fn init(w: &WebviewWindow) -> Result<(), String> {
    use gtk::gdk::WindowState;
    use gtk::prelude::*;
    use tauri::Manager;
    use tauri::window::Color;
    use tauri_specta::Event;

    let gw = w.gtk_window().map_err(|e| e.to_string())?;
    if gw.titlebar().is_none() {
        // X11: setting a title bar now would re-realize the webview.
        return Ok(());
    }
    if gw.is_realized() {
        return Err("window already realized".to_owned());
    }
    // Without an alpha channel the shadow is drawn as a black border.
    let Some(visual) = GtkWindowExt::screen(&gw).and_then(|s| s.rgba_visual()) else {
        return Ok(());
    };
    gw.set_visual(Some(&visual));
    gw.set_decorated(true);

    // The shadow follows the decoration's radius. This provider outranks the theme, so it also
    // has to square the states the theme squares.
    let css = gtk::CssProvider::new();
    css.load_from_data(
        format!(
            "decoration {{ border-radius: {RADIUS}px; }}
             .maximized decoration, .fullscreen decoration, .tiled decoration,
             .tiled-top decoration, .tiled-right decoration, .tiled-bottom decoration,
             .tiled-left decoration {{ border-radius: 0; }}"
        )
        .as_bytes(),
    )
    .map_err(|e| e.to_string())?;
    gw.style_context()
        .add_provider(&css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    // Clears both the GTK window's and the webview's background, so the corners show through.
    w.set_background_color(Some(Color(0, 0, 0, 0)))
        .map_err(|e| e.to_string())?;

    let square = WindowState::MAXIMIZED
        | WindowState::FULLSCREEN
        | WindowState::TILED
        | WindowState::TOP_TILED
        | WindowState::RIGHT_TILED
        | WindowState::BOTTOM_TILED
        | WindowState::LEFT_TILED;
    let app = w.app_handle().clone();
    app.state::<Frame>().0.store(RADIUS, Ordering::Relaxed);
    gw.connect_window_state_event(move |_, e| {
        let radius = if e.new_window_state().intersects(square) {
            0
        } else {
            RADIUS
        };
        if app.state::<Frame>().0.swap(radius, Ordering::Relaxed) != radius
            && let Err(e) = (WindowFrame { radius }).emit(&app)
        {
            tracing::warn!("could not emit window frame: {e}");
        }
        gtk::glib::Propagation::Proceed
    });
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn init(_: &WebviewWindow) -> Result<(), String> {
    Ok(())
}
