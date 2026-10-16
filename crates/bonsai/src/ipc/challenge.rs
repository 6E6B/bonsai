//! The challenge dialog: a two-step code or captcha that Roblox put in front of an action.
//! The dialog opens itself from `Shell::challenge`; these commands are its steps, so they are
//! not separate palette entries.

use std::sync::Arc;

use rbx_core::Core;
use rbx_core::challenge::{ChallengePrompt, PromptKind};
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, State};

use super::Id;

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeView {
    /// Identifies this check in the dialog's replies.
    pub seq: Id,
    /// The account Roblox is checking.
    pub account: Id,
    pub kind: ChallengeKind,
    /// Waiting on Roblox.
    pub busy: bool,
    pub error: Option<String>,
    pub notice: Option<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ChallengeKind {
    /// A 6-digit code. `method` and `methods` are `authenticator` or `email`.
    #[serde(rename_all = "camelCase")]
    Code {
        method: String,
        methods: Vec<String>,
        remember_option: bool,
    },
    /// A puzzle, solved in its own window.
    Captcha,
}

pub(super) fn view(p: &ChallengePrompt) -> ChallengeView {
    ChallengeView {
        seq: Id(p.seq),
        account: Id(p.account.0),
        kind: match &p.kind {
            PromptKind::TwoStep {
                method,
                methods,
                remember_option,
            } => ChallengeKind::Code {
                method: method.key().to_owned(),
                methods: methods.iter().map(|m| m.key().to_owned()).collect(),
                remember_option: *remember_option,
            },
            PromptKind::Captcha => ChallengeKind::Captcha,
        },
        busy: p.busy,
        error: p.error.clone(),
        notice: p.notice.clone(),
    }
}

/// Checks a two-step code. A wrong code shows in the dialog.
#[tauri::command]
#[specta::specta]
pub fn challenge_submit_code(
    core: State<'_, Arc<Core>>,
    seq: Id,
    code: String,
    remember: bool,
) -> Result<(), String> {
    core.challenge_submit_code(seq.0, &code, remember)
}

/// Switches between `authenticator` and `email`. Picking `email` sends a new code.
#[tauri::command]
#[specta::specta]
pub fn challenge_use_method(core: State<'_, Arc<Core>>, seq: Id, method: String) -> Result<(), String> {
    core.challenge_use_method(seq.0, &method)
}

/// Closes the dialog. The action it held fails.
#[tauri::command]
#[specta::specta]
pub fn challenge_cancel(app: AppHandle, core: State<'_, Arc<Core>>, seq: Id) {
    crate::captcha::close(&app);
    core.challenge_cancel(seq.0);
}

/// Opens the puzzle window for a captcha check.
#[tauri::command]
#[specta::specta]
pub fn open_captcha(app: AppHandle, core: State<'_, Arc<Core>>, seq: Id) -> Result<(), String> {
    crate::captcha::open(&app, &core, seq.0)
}
