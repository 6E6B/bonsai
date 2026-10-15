//! Turns rbx-core's repaint calls into at most one [`StoreChanged`] event per frame, so a
//! burst of background writes costs the UI one refetch instead of hundreds.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use rbx_core::Repaint;
use tauri::AppHandle;
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
            }
        })?;
    let thread = worker.thread().clone();
    Ok(Arc::new(move || {
        if !dirty.swap(true, Ordering::AcqRel) {
            thread.unpark();
        }
    }))
}
