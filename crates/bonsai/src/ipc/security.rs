//! Settings > Security: the password, two-step methods, recovery codes and passkeys. The
//! dialogs that the `security.*` commands open call these; they are not separate palette
//! entries. Passwords go straight to bonsai-core and are not kept here.

use std::sync::Arc;

use bonsai_core::security::{SecurityFlow, SecurityStep, SecurityTask};
use bonsai_core::{AccountId, Core};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::State;

use super::Id;

#[derive(Clone, Debug, Default, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SecurityView {
    pub loading: bool,
    pub error: Option<String>,
    pub email: bool,
    pub authenticator: bool,
    pub sms: bool,
    pub security_key: bool,
    /// Unused recovery codes, when Roblox said.
    pub recovery_codes: Option<u32>,
    /// ISO date the recovery codes were made.
    pub recovery_created: Option<String>,
    pub passkeys: Vec<String>,
}

/// Two-step methods, recovery codes and passkeys of the active account.
#[tauri::command]
#[specta::specta]
pub fn security_status(core: State<'_, Arc<Core>>) -> SecurityView {
    let r = core.acting(None).map(|a| core.security(a)).unwrap_or_default();
    let s = r.data.as_deref().cloned().unwrap_or_default();
    SecurityView {
        loading: r.first_load(),
        error: r.error.clone(),
        email: s.email,
        authenticator: s.authenticator,
        sms: s.sms,
        security_key: s.security_key,
        recovery_codes: s.recovery_codes,
        recovery_created: s.recovery_created,
        passkeys: s.passkeys,
    }
}

fn acting(core: &Core) -> Result<AccountId, String> {
    core.acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())
}

/// Changes the password of the active account. The result arrives as a toast.
#[tauri::command]
#[specta::specta]
pub fn change_password(
    core: State<'_, Arc<Core>>,
    current: String,
    replacement: String,
) -> Result<(), String> {
    core.change_password(acting(&core)?, &current, &replacement)
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum CodeSource {
    Email,
    Authenticator,
}

/// Turns email codes on or off, or the authenticator app off. Turning the authenticator on
/// takes the setup dialog (`security_open`).
#[tauri::command]
#[specta::specta]
pub fn security_set(
    core: State<'_, Arc<Core>>,
    source: CodeSource,
    on: bool,
    password: String,
) -> Result<(), String> {
    let a = acting(&core)?;
    match (source, on) {
        (CodeSource::Email, on) => core.set_email_codes(a, on, &password),
        (CodeSource::Authenticator, false) => core.authenticator_off(a, &password),
        (CodeSource::Authenticator, true) => Err("Turn it on from the setup dialog.".into()),
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum SecurityTaskView {
    Authenticator,
    RecoveryCodes,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum SecurityStepView {
    Password,
    /// Type `manualKey` into an authenticator app, then enter its code.
    #[serde(rename_all = "camelCase")]
    Key {
        manual_key: String,
    },
    /// Recovery codes to write down; empty when Roblox made none.
    Done {
        codes: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SecurityFlowView {
    /// Identifies this attempt in the dialog's replies.
    pub seq: Id,
    pub task: SecurityTaskView,
    pub step: SecurityStepView,
    pub busy: bool,
    pub error: Option<String>,
}

pub(super) fn view(p: &SecurityFlow) -> SecurityFlowView {
    SecurityFlowView {
        seq: Id(p.seq),
        task: match p.task {
            SecurityTask::Authenticator => SecurityTaskView::Authenticator,
            SecurityTask::RecoveryCodes => SecurityTaskView::RecoveryCodes,
        },
        step: match &p.step {
            SecurityStep::Password => SecurityStepView::Password,
            SecurityStep::Key { manual_key } => SecurityStepView::Key {
                manual_key: manual_key.clone(),
            },
            SecurityStep::Done { codes } => SecurityStepView::Done { codes: codes.clone() },
        },
        busy: p.busy,
        error: p.error.clone(),
    }
}

/// Opens the authenticator setup or the recovery codes dialog. Returns the attempt's `seq`.
#[tauri::command]
#[specta::specta]
pub fn security_open(core: State<'_, Arc<Core>>, task: SecurityTaskView) -> Result<Id, String> {
    let task = match task {
        SecurityTaskView::Authenticator => SecurityTask::Authenticator,
        SecurityTaskView::RecoveryCodes => SecurityTask::RecoveryCodes,
    };
    core.security_open(acting(&core)?, task).map(Id)
}

/// The password step of the open dialog.
#[tauri::command]
#[specta::specta]
pub fn security_password(core: State<'_, Arc<Core>>, seq: Id, password: String) -> Result<(), String> {
    core.security_password(seq.0, &password)
}

/// The authenticator code step of the open dialog.
#[tauri::command]
#[specta::specta]
pub fn security_code(core: State<'_, Arc<Core>>, seq: Id, code: String) -> Result<(), String> {
    core.security_code(seq.0, &code)
}

/// Closes the dialog. Core forgets the password and setup token.
#[tauri::command]
#[specta::specta]
pub fn security_close(core: State<'_, Arc<Core>>, seq: Id) {
    core.security_close(seq.0);
}

/// Copies what the open dialog shows: the authenticator key, or the recovery codes.
#[tauri::command]
#[specta::specta]
pub fn security_copy(app: tauri::AppHandle, core: State<'_, Arc<Core>>, seq: Id) -> Result<(), String> {
    let text = match core
        .store()
        .security
        .as_ref()
        .filter(|p| p.seq == seq.0)
        .map(|p| &p.step)
    {
        Some(SecurityStep::Key { manual_key }) => manual_key.clone(),
        Some(SecurityStep::Done { codes }) if !codes.is_empty() => codes.join("\n"),
        _ => return Err("Nothing to copy.".into()),
    };
    super::copy(&app, &core, text, "Copied.")
}
