//! Signed application updates. This isolated transport never receives Roblox credentials.
//! The plugin verifies the package before it becomes installable; installation is explicit.

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bonsai_core::Core;
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_updater::{Update, UpdaterExt};

const ENDPOINT: &str = "https://github.com/6E6B/bonsai/releases/latest/download/latest.json";
const PUBLIC_KEY: &str = match option_env!("BONSAI_UPDATER_PUBLIC_KEY") {
    Some(key) => key,
    None => "",
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    Unavailable,
    Idle,
    Checking,
    Available,
    Downloading,
    Verifying,
    Ready,
    Installing,
}

#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    pub phase: Phase,
    pub current_version: String,
    pub version: Option<String>,
    pub notes: Option<String>,
    #[specta(type = specta_typescript::Number)]
    pub downloaded: f64,
    pub total: Option<f64>,
    pub checked_at: Option<f64>,
    pub error: Option<String>,
    pub unavailable_reason: Option<String>,
}

struct Inner {
    status: UpdateStatus,
    update: Option<Update>,
    bytes: Option<Arc<Vec<u8>>>,
}

pub struct Updater(Mutex<Inner>);

impl Updater {
    fn change(&self, f: impl FnOnce(&mut Inner)) {
        f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()));
    }
}

#[tauri::command]
#[specta::specta]
pub fn update_status(updater: State<'_, Updater>) -> UpdateStatus {
    updater.0.lock().unwrap_or_else(|e| e.into_inner()).status.clone()
}

pub fn start(app: &AppHandle) -> tauri::Result<()> {
    let reason = if cfg!(debug_assertions) {
        Some("Development build. Updates are available in installed release builds.")
    } else if PUBLIC_KEY.trim().is_empty() {
        Some("This build has no update signing key. Install an official Bonsai release to receive updates.")
    } else if cfg!(target_os = "linux") && std::env::var_os("APPIMAGE").is_none() {
        Some("Update Bonsai through your package manager. In-app updates require the AppImage edition.")
    } else {
        None
    };
    if reason.is_none() {
        app.plugin(
            tauri_plugin_updater::Builder::new()
                .pubkey(PUBLIC_KEY.trim())
                .build(),
        )?;
    }
    app.manage(Updater(Mutex::new(Inner {
        status: UpdateStatus {
            phase: if reason.is_some() {
                Phase::Unavailable
            } else {
                Phase::Idle
            },
            current_version: app.package_info().version.to_string(),
            version: None,
            notes: None,
            downloaded: 0.0,
            total: None,
            checked_at: None,
            error: None,
            unavailable_reason: reason.map(str::to_owned),
        },
        update: None,
        bytes: None,
    })));
    if reason.is_none() {
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            // Let the initial window and account cache load first. Also works in tray mode.
            tokio::time::sleep(Duration::from_secs(15)).await;
            loop {
                if app.state::<Arc<Core>>().settings().update_checks {
                    let _ = check(&app);
                }
                tokio::time::sleep(Duration::from_secs(6 * 60 * 60)).await;
            }
        });
    }
    Ok(())
}

pub fn check(app: &AppHandle) -> Result<(), String> {
    let mut begin = false;
    app.state::<Updater>().change(|s| {
        // Never discard a pending update or race a download/install with another check.
        if s.status.phase == Phase::Idle {
            s.status.phase = Phase::Checking;
            s.status.error = None;
            begin = true;
        }
    });
    if !begin {
        return Ok(());
    }
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let result = async {
            app.updater_builder()
                .endpoints(vec![ENDPOINT.parse()?])?
                .timeout(Duration::from_secs(30))
                .build()?
                .check()
                .await
        }
        .await;
        app.state::<Updater>().change(|s| match result {
            Ok(mut update) => {
                if update
                    .as_ref()
                    .is_some_and(|u| u.download_url.scheme() != "https")
                {
                    s.status.phase = Phase::Idle;
                    s.status.error =
                        Some("The release contains an insecure download URL. Nothing was downloaded.".into());
                    return;
                }
                if let Some(u) = &mut update {
                    u.timeout = Some(Duration::from_secs(30 * 60));
                }
                s.status.checked_at = Some(now());
                s.status.phase = if update.is_some() {
                    Phase::Available
                } else {
                    Phase::Idle
                };
                s.status.version = update.as_ref().map(|u| u.version.clone());
                s.status.notes = update.as_ref().and_then(|u| u.body.clone());
                s.update = update;
            }
            Err(e) => {
                tracing::warn!("update check failed: {e}");
                s.status.phase = Phase::Idle;
                s.status.error =
                    Some("Could not check for updates. Check your connection and try again.".into());
            }
        });
        if app.state::<Arc<Core>>().settings().update_downloads {
            let _ = download(&app);
        }
    });
    Ok(())
}

pub fn download(app: &AppHandle) -> Result<(), String> {
    let mut update = None;
    app.state::<Updater>().change(|s| {
        if s.status.phase == Phase::Available {
            update = s.update.clone();
            if update.is_some() {
                s.status.phase = Phase::Downloading;
                s.status.error = None;
                s.status.downloaded = 0.0;
                s.status.total = None;
            }
        }
    });
    let update = update.ok_or("Check for an update before downloading.")?;
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        let result = update
            .download(
                |chunk, total| {
                    app.state::<Updater>().change(|s| {
                        s.status.downloaded += chunk as f64;
                        s.status.total = total.map(|n| n as f64);
                    });
                },
                || {
                    app.state::<Updater>()
                        .change(|s| s.status.phase = Phase::Verifying)
                },
            )
            .await;
        app.state::<Updater>().change(|s| match result {
            Ok(bytes) => {
                s.bytes = Some(Arc::new(bytes));
                s.status.phase = Phase::Ready;
            }
            Err(e) => {
                tracing::warn!("update download or signature verification failed: {e}");
                s.status.phase = Phase::Available;
                s.status.error = Some("Download or signature verification failed. Nothing was installed. Try downloading again.".into());
            }
        });
    });
    Ok(())
}

pub fn install(app: &AppHandle) -> Result<(), String> {
    let mut pending = None;
    app.state::<Updater>().change(|s| {
        if s.status.phase == Phase::Ready
            && let (Some(update), Some(bytes)) = (&s.update, &s.bytes)
        {
            pending = Some((update.clone(), bytes.clone()));
            s.status.phase = Phase::Installing;
            s.status.error = None;
        }
    });
    let (update, bytes) = pending.ok_or("Download and verify the update before installing.")?;
    let app = app.clone();
    // Installation does disk/process work. Never block the IPC or async executor threads.
    tauri::async_runtime::spawn_blocking(move || {
        if let Err(e) = update.install(bytes.as_slice()) {
            tracing::warn!("update installation failed: {e}");
            app.state::<Updater>().change(|s| {
                s.status.phase = Phase::Ready;
                s.status.error = Some(
                    "Could not install the update. Check disk space and write permissions, then retry."
                        .into(),
                );
            });
            return;
        }
        // Windows installers exit the running app themselves. macOS and AppImage need this.
        app.restart();
    });
    Ok(())
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
        * 1000.0
}
