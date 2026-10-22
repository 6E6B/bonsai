//! OS session notifications, independent of webview focus and JavaScript timers.
use std::sync::Arc;

pub enum Event {
    Lock,
    Unavailable(&'static str),
}
pub type Callback = Arc<dyn Fn(Event) + Send + Sync>;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub fn start(callback: Callback) {
    #[cfg(target_os = "linux")]
    linux::start(callback);
    #[cfg(target_os = "macos")]
    macos::start(callback);
    #[cfg(target_os = "windows")]
    windows::start(callback);
}
