//! The title bar's window buttons on Linux. The UI draws its own buttons, but on Linux it draws
//! them with the desktop's Freedesktop symbolic icons (`window-close-symbolic` and friends), in
//! the order and on the side the desktop's button layout asks for. Breeze gets KDE-style
//! buttons; any other theme gets Adwaita-style ones, like Firefox does. When no icon theme has
//! the icons, the UI keeps its bundled icons.
//!
//! `wicon://` serves the icons: URL path `/<minimize|maximize|restore|close>`. The UI uses them
//! as CSS masks, so they take the theme's text color.
//!
//! The icons and layout are read once at startup.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder, Window, WindowEvent};
use tauri_specta::Event;

use crate::ipc::{ButtonStyle, WindowButton, WindowButtons, WindowMaximized};

pub const SCHEME: &str = "wicon";

/// Icon names served over `wicon://`, with the Freedesktop name each one looks up.
const ICONS: [(&str, &str); 4] = [
    ("minimize", "window-minimize-symbolic"),
    ("maximize", "window-maximize-symbolic"),
    ("restore", "window-restore-symbolic"),
    ("close", "window-close-symbolic"),
];

/// What the title bar draws, read at startup, and whether the window is maximized now.
pub struct Titlebar {
    buttons: WindowButtons,
    /// Files for [`ICONS`], in the same order. Empty when the theme lacks any of them.
    files: Vec<PathBuf>,
    maximized: AtomicBool,
}

impl Default for Titlebar {
    fn default() -> Self {
        Self {
            buttons: WindowButtons {
                style: ButtonStyle::Builtin,
                start: Vec::new(),
                end: vec![
                    WindowButton::Minimize,
                    WindowButton::Maximize,
                    WindowButton::Close,
                ],
            },
            files: Vec::new(),
            maximized: AtomicBool::new(false),
        }
    }
}

impl Titlebar {
    /// Reads the desktop's icons and button layout. Call on the main thread once GTK is up.
    #[cfg(target_os = "linux")]
    pub fn read() -> Self {
        use gtk::prelude::*;

        let mut t = Self::default();
        let Some(settings) = gtk::Settings::default() else {
            return t;
        };
        if let Some(layout) = settings.gtk_decoration_layout() {
            (t.buttons.start, t.buttons.end) = parse_layout(&layout);
        }
        let Some(theme) = gtk::IconTheme::default() else {
            return t;
        };
        let files: Vec<PathBuf> = ICONS
            .iter()
            .filter_map(|(_, name)| {
                theme
                    .lookup_icon(name, 16, gtk::IconLookupFlags::empty())
                    .and_then(|i| i.filename())
                    .filter(|f| content_type(f).is_some())
            })
            .collect();
        if files.len() != ICONS.len() {
            return t;
        }
        let names = [
            settings.gtk_theme_name().unwrap_or_default(),
            settings.gtk_icon_theme_name().unwrap_or_default(),
        ];
        t.buttons.style = if names.iter().any(|n| n.to_lowercase().starts_with("breeze")) {
            ButtonStyle::Breeze
        } else {
            ButtonStyle::Adwaita
        };
        t.files = files;
        t
    }

    #[cfg(not(target_os = "linux"))]
    pub fn read() -> Self {
        Self::default()
    }

    pub fn buttons(&self) -> WindowButtons {
        self.buttons.clone()
    }

    pub fn set_maximized(&self, maximized: bool) {
        self.maximized.store(maximized, Ordering::Relaxed);
    }

    pub fn maximized(&self) -> bool {
        self.maximized.load(Ordering::Relaxed)
    }
}

/// Splits a GTK decoration layout like `icon:minimize,maximize,close` into the buttons at the
/// start and at the end of the bar. Entries other than the three buttons (`icon`, `menu`,
/// `appmenu`, `spacer`) are skipped. With no colon, everything sits at the start, as in GTK.
#[cfg(any(target_os = "linux", test))]
fn parse_layout(layout: &str) -> (Vec<WindowButton>, Vec<WindowButton>) {
    let side = |s: &str| {
        s.split(',')
            .filter_map(|b| match b.trim() {
                "minimize" => Some(WindowButton::Minimize),
                "maximize" => Some(WindowButton::Maximize),
                "close" => Some(WindowButton::Close),
                _ => None,
            })
            .collect()
    };
    match layout.split_once(':') {
        Some((start, end)) => (side(start), side(end)),
        None => (side(layout), Vec::new()),
    }
}

fn content_type(file: &std::path::Path) -> Option<&'static str> {
    match file.extension()?.to_str()? {
        "svg" => Some("image/svg+xml"),
        "png" => Some("image/png"),
        _ => None,
    }
}

/// Tells the UI when the main window is maximized or restored, so it can swap the maximize
/// icon for the restore icon.
pub fn on_window_event(w: &Window, e: &WindowEvent) {
    if !matches!(e, WindowEvent::Resized(_)) || w.label() != "main" {
        return;
    }
    let Some(t) = w.try_state::<Titlebar>() else {
        return;
    };
    let maximized = w.is_maximized().unwrap_or(false);
    if t.maximized.swap(maximized, Ordering::Relaxed) != maximized
        && let Err(e) = (WindowMaximized { maximized }).emit(w.app_handle())
    {
        tracing::warn!("could not emit window maximized: {e}");
    }
}

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    let Some(i) = ICONS
        .iter()
        .position(|(n, _)| *n == req.uri().path().trim_start_matches('/'))
    else {
        return res.respond(status(StatusCode::BAD_REQUEST));
    };
    let Some(file) = ctx
        .app_handle()
        .try_state::<Titlebar>()
        .and_then(|t| t.files.get(i).cloned())
    else {
        return res.respond(status(StatusCode::NOT_FOUND));
    };
    tauri::async_runtime::spawn_blocking(move || {
        let r = match (std::fs::read(&file), content_type(&file)) {
            (Ok(bytes), Some(ty)) => Response::builder()
                .header(header::CONTENT_TYPE, ty)
                // CSS masks are fetched in CORS mode, and the page is on another origin.
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .header(header::CACHE_CONTROL, "max-age=3600")
                .body(bytes)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR)),
            _ => status(StatusCode::NOT_FOUND),
        };
        res.respond(r);
    });
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut r = Response::new(Vec::new());
    *r.status_mut() = code;
    r
}

#[cfg(test)]
mod tests {
    use super::parse_layout;
    use crate::ipc::WindowButton::{Close, Maximize, Minimize};

    #[test]
    fn parses_decoration_layouts() {
        assert_eq!(
            parse_layout("icon:minimize,maximize,close"),
            (vec![], vec![Minimize, Maximize, Close])
        );
        // GNOME's default.
        assert_eq!(parse_layout("appmenu:close"), (vec![], vec![Close]));
        assert_eq!(
            parse_layout("close,minimize,maximize:menu"),
            (vec![Close, Minimize, Maximize], vec![])
        );
        assert_eq!(
            parse_layout("close, spacer ,maximize"),
            (vec![Close, Maximize], vec![])
        );
        assert_eq!(parse_layout(""), (vec![], vec![]));
    }
}
