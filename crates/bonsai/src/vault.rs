//! IPC and process-session lifecycle for the account vault. Cryptography lives in core.
use bonsai_core::{Core, vault::Status};
use serde::Serialize;
use specta::Type;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

#[derive(Default)]
pub struct SystemLockWarning(std::sync::Mutex<Option<String>>);

#[derive(Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct VaultView {
    pub status: String,
    pub error: Option<String>,
    pub retry_seconds: u32,
    pub locked: bool,
    pub system_lock_warning: Option<String>,
}
#[tauri::command]
#[specta::specta]
pub fn vault_status(core: State<'_, Arc<Core>>, warning: State<'_, SystemLockWarning>) -> VaultView {
    let status = match core.vault_status() {
        Status::Disabled => "disabled",
        Status::Locked => "locked",
        Status::Unlocked => "unlocked",
        Status::Unavailable => "unavailable",
    };
    VaultView {
        status: status.into(),
        error: core.vault_error(),
        retry_seconds: core.vault_retry_seconds(),
        locked: core.accounts_locked(),
        system_lock_warning: warning.0.lock().ok().and_then(|w| w.clone()),
    }
}
#[tauri::command]
#[specta::specta]
pub async fn vault_submit(
    app: AppHandle,
    core: State<'_, Arc<Core>>,
    action: String,
    current: String,
    next: String,
) -> Result<(), String> {
    // Do not copy secrets into the command registry, history, events, settings or logs.
    let current = zeroize::Zeroizing::new(current);
    let next = zeroize::Zeroizing::new(next);
    if current.len() > 256 || next.len() > 256 {
        return Err("Unlock secret exceeds 256 bytes.".into());
    }
    let core = core.inner().clone();
    let forgetting = action == "forget";
    let result = tauri::async_runtime::spawn_blocking(move || core.vault_action(&action, &current, &next))
        .await
        .map_err(|_| "Vault operation stopped unexpectedly. Protection remains enabled.".to_owned())?;
    if forgetting && result.is_ok() {
        restart_session(&app);
    }
    result
}
#[tauri::command]
#[specta::specta]
pub fn vault_activity(core: State<'_, Arc<Core>>) {
    core.vault_activity();
}

pub fn lock(app: &AppHandle, core: &Core) -> Result<(), String> {
    if core.vault_status() == Status::Disabled {
        return Err("PIN protection is disabled.".into());
    }
    if !core.close_vault_session() {
        return Ok(());
    }
    restart_session(app);
    Ok(())
}

fn restart_session(app: &AppHandle) {
    let handle = app.clone();
    if app.run_on_main_thread(move || finish_lock(&handle)).is_err() {
        std::process::exit(1);
    }
}

fn finish_lock(app: &AppHandle) {
    crate::signin::close(app);
    crate::captcha::close(app);
    // Hide immediately. Killing this process session also destroys pending tasks, plugin VMs
    // and all private DOM state, before another unlock is possible.
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    app.restart();
}

/// Backend inactivity timer cannot be bypassed by pausing JavaScript.
/// Native OS session-lock integration is separate from ordinary window focus changes.
pub fn start_idle(app: AppHandle) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if let Some(core) = app.try_state::<Arc<Core>>()
                && core.vault_idle()
            {
                let _ = lock(&app, &core);
            }
        }
    });
}

pub fn start_system_lock(app: AppHandle) {
    use crate::session_lock::Event;
    use tauri_specta::Event as _;
    crate::session_lock::start(Arc::new(move |event| match event {
        Event::Lock => {
            let Some(core) = app.try_state::<Arc<Core>>() else {
                return;
            };
            if !core.vault_session_live() {
                return;
            }
            // Close admission on the notification thread before queueing main-thread UI
            // cleanup. A sleeping/blocked event loop cannot keep accepting account work.
            if !core.close_vault_session() {
                return;
            }
            restart_session(&app);
        }
        Event::Unavailable(message) => {
            if let Some(warning) = app.try_state::<SystemLockWarning>()
                && let Ok(mut warning) = warning.0.lock()
            {
                *warning = Some(format!("{message} Use Lock now before leaving Bonsai."));
            }
            let _ = crate::ipc::StoreChanged.emit(&app);
        }
    }));
}
