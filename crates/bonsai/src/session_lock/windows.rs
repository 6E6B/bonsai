use super::{Callback, Event};
use std::sync::OnceLock;
use windows_sys::Win32::{
    Foundation::*,
    System::{LibraryLoader::GetModuleHandleW, RemoteDesktop::*},
    UI::WindowsAndMessaging::*,
};

static CALLBACK: OnceLock<Callback> = OnceLock::new();

pub fn start(callback: Callback) {
    if CALLBACK.set(callback.clone()).is_err() {
        return;
    }
    std::thread::spawn(move || {
        // This hidden top-level window receives WTS and power broadcasts. No pointers
        // to Rust-owned data cross the window procedure, and its thread owns the HWND.
        let result = unsafe { watch() };
        if result.is_err() {
            callback(Event::Unavailable(
                "Could not monitor Windows session notifications.",
            ));
            callback(Event::Lock);
        }
    });
}

unsafe fn watch() -> Result<(), ()> {
    let instance = unsafe { GetModuleHandleW(std::ptr::null()) };
    let class = windows_sys::core::w!("BonsaiVaultSessionMonitor");
    let wc = WNDCLASSW {
        lpfnWndProc: Some(window_proc),
        hInstance: instance,
        lpszClassName: class,
        ..Default::default()
    };
    if unsafe { RegisterClassW(&wc) } == 0 {
        return Err(());
    }
    let window = unsafe {
        CreateWindowExW(
            0,
            class,
            class,
            0,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null(),
        )
    };
    if window.is_null() {
        return Err(());
    }
    // Remote Desktop Services can still be starting during an autostart launch.
    let mut registered = false;
    for _ in 0..10 {
        if unsafe { WTSRegisterSessionNotification(window, NOTIFY_FOR_THIS_SESSION) } != 0 {
            registered = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    if !registered {
        unsafe {
            DestroyWindow(window);
        }
        return Err(());
    }
    let mut message = MSG::default();
    loop {
        let result = unsafe { GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) };
        if result <= 0 {
            break;
        }
        unsafe {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    unsafe {
        WTSUnRegisterSessionNotification(window);
        DestroyWindow(window);
    }
    Err(()) // Losing the monitor must not silently leave an unlocked session behind.
}

unsafe extern "system" fn window_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let session_lock = message == WM_WTSSESSION_CHANGE
        && matches!(
            wparam as u32,
            WTS_SESSION_LOCK | WTS_SESSION_LOGOFF | WTS_CONSOLE_DISCONNECT | WTS_REMOTE_DISCONNECT
        );
    let power = message == WM_POWERBROADCAST
        && matches!(
            wparam as u32,
            PBT_APMSUSPEND | PBT_APMRESUMEAUTOMATIC | PBT_APMRESUMESUSPEND
        );
    if (session_lock || power)
        && let Some(callback) = CALLBACK.get()
    {
        // Never unwind through the system's C ABI.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(Event::Lock)));
    }
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}
