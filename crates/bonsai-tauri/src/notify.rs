//! Turns rbx-core's repaint calls into at most one [`StoreChanged`] event per frame, so a
//! burst of background writes costs the UI one refetch instead of hundreds. Also shows
//! rbx-core's desktop alerts as system notifications while the window is in the background.

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

use rbx_core::{Core, Repaint};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use tauri_specta::Event;
use time::{OffsetDateTime, UtcOffset};

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

/// The machine's UTC offset, read once at startup. Reading it later is unsound on Linux once
/// other threads exist, so a daylight saving change shows after a restart.
static LOCAL_OFFSET: OnceLock<UtcOffset> = OnceLock::new();

/// Call first thing in `run`, before any thread starts.
pub fn capture_local_offset() {
    match UtcOffset::current_local_offset() {
        Ok(o) => {
            let _ = LOCAL_OFFSET.set(o);
        }
        Err(e) => tracing::warn!("local time unknown, quiet hours use UTC: {e}"),
    }
}

fn local_hour() -> u8 {
    let offset = LOCAL_OFFSET.get().copied().unwrap_or(UtcOffset::UTC);
    OffsetDateTime::now_utc().to_offset(offset).hour()
}

/// Show waiting alerts as system notifications. They are dropped during quiet hours, while
/// playing if muted, and while the app is in front unless that is turned on (the badges and
/// lists already say it then).
fn show_alerts(app: &AppHandle) {
    let Some(core) = app.try_state::<Arc<Core>>() else {
        return;
    };
    let alerts = core.take_alerts();
    if alerts.is_empty() {
        return;
    }
    let settings = core.settings();
    if settings.quiet.covers(local_hour()) || (settings.mute_in_game && core.playing()) {
        return;
    }
    let focused = app
        .get_webview_window("main")
        .is_some_and(|w| w.is_focused().unwrap_or(false));
    if focused && !settings.notify_in_front {
        return;
    }
    for a in alerts {
        let mut n = app.notification().builder().title(&a.title).body(&a.body);
        if settings.notify_sound {
            n = n.sound(SOUND);
        }
        if let Err(e) = n.show() {
            tracing::warn!("could not show a notification: {e}");
        }
    }
}

/// The system's own message sound.
#[cfg(target_os = "linux")]
const SOUND: &str = "message-new-instant";
#[cfg(not(target_os = "linux"))]
const SOUND: &str = "Default";
