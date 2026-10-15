//! Turns rbx-core's repaint calls into at most one [`StoreChanged`] event per frame, so a
//! burst of background writes costs the UI one refetch instead of hundreds. Also shows
//! rbx-core's desktop alerts as system notifications while the window is in the background.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use rbx_core::{Core, Repaint};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use tauri_specta::Event;

use crate::ipc::StoreChanged;

const FRAME: Duration = Duration::from_millis(16);

pub fn spawn(app: AppHandle) -> io::Result<Repaint> {
    let dirty = Arc::new(AtomicBool::new(false));
    let flag = dirty.clone();
    let worker = thread::Builder::new()
        .name("store-notify".into())
        .spawn(move || {
            loop {
                thread::park();
                // Let the rest of the burst land, then send one event for all of it.
                thread::sleep(FRAME);
                if flag.swap(false, Ordering::AcqRel)
                    && let Err(e) = StoreChanged.emit(&app)
                {
                    tracing::warn!("could not emit store change: {e}");
                }
                show_alerts(&app);
            }
        })?;
    let thread = worker.thread().clone();
    Ok(Arc::new(move || {
        if !dirty.swap(true, Ordering::AcqRel) {
            thread.unpark();
        }
    }))
}

/// Show waiting alerts as system notifications, unless the app is in front (the badges and
/// lists already say it then).
fn show_alerts(app: &AppHandle) {
    let Some(core) = app.try_state::<Arc<Core>>() else {
        return;
    };
    let alerts = core.take_alerts();
    if alerts.is_empty() {
        return;
    }
    let focused = app
        .get_webview_window("main")
        .is_some_and(|w| w.is_focused().unwrap_or(false));
    if focused {
        return;
    }
    for a in alerts {
        if let Err(e) = app.notification().builder().title(&a.title).body(&a.body).show() {
            tracing::warn!("could not show a notification: {e}");
        }
    }
}
