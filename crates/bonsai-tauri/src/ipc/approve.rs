//! The "Sign in another device" dialog behind `account.approve_code`: enter the code another
//! device shows, see which device it is, then confirm or cancel. These commands are steps of
//! that dialog, so they are not separate palette entries.

use std::sync::Arc;

use rbx_core::Core;
use rbx_core::signin::{ApprovePrompt, ApproveStep};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::Id;

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ApproveView {
    /// Identifies this attempt in the dialog's replies.
    pub seq: Id,
    pub step: ApproveStepView,
    /// Waiting on Roblox.
    pub busy: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum ApproveStepView {
    Enter,
    /// `device` like "Chrome on Windows"; `location` may be empty.
    Confirm {
        device: String,
        location: String,
    },
    Done,
}

pub(super) fn view(p: &ApprovePrompt) -> ApproveView {
    ApproveView {
        seq: Id(p.seq),
        step: match &p.step {
            ApproveStep::Enter => ApproveStepView::Enter,
            ApproveStep::Confirm { device, location } => ApproveStepView::Confirm {
                device: device.clone(),
                location: location.clone(),
            },
            ApproveStep::Done => ApproveStepView::Done,
        },
        busy: p.busy,
        error: p.error.clone(),
    }
}

/// Enters a code for the active account. Returns the attempt's `seq`.
#[tauri::command]
#[specta::specta]
pub fn approve_enter(core: State<'_, Arc<Core>>, code: String) -> Result<Id, String> {
    let a = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    core.approve_enter(a, &code).map(Id)
}

/// Approves the code the dialog shows.
#[tauri::command]
#[specta::specta]
pub fn approve_confirm(core: State<'_, Arc<Core>>, seq: Id) -> Result<(), String> {
    core.approve_confirm(seq.0)
}

/// Closes the dialog. An entered code that wasn't approved is cancelled.
#[tauri::command]
#[specta::specta]
pub fn approve_close(core: State<'_, Arc<Core>>, seq: Id) {
    core.approve_close(seq.0);
}
