//! Streamer mode: what the UI hides, the recording apps seen and the question on screen.

use bonsai_core::Core;
use bonsai_core::commands::CommandId;
use serde::Serialize;
use specta::Type;

use super::{Id, Invocation};

/// Asks to turn streamer mode on (`start`) or off because `app` opened or closed. Shown once
/// per `seq`.
#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StreamerPromptView {
    pub seq: Id,
    pub app: String,
    pub start: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StreamerView {
    pub on: bool,
    /// `ask`, `auto` or `off`.
    pub detect: String,
    /// Recording apps open now, by name.
    pub running: Vec<String>,
    /// Apps the user said not to ask about.
    pub ignored: Vec<String>,
    pub hide_messages: bool,
    pub hide_capture: bool,
    /// This system can keep the window out of screen capture.
    pub capture_supported: bool,
    pub prompt: Option<StreamerPromptView>,
}

/// Whether `set_content_protected` does anything here. Wayland and X11 capture can't be
/// blocked from inside the window.
pub const CAPTURE_SUPPORTED: bool = cfg!(any(target_os = "windows", target_os = "macos"));

pub(super) fn view(core: &Core) -> StreamerView {
    let p = core.settings().streamer;
    let s = core.store();
    StreamerView {
        on: p.on,
        detect: p.detect().name().to_owned(),
        running: s.streamer.running.clone(),
        ignored: p.ignored.clone(),
        hide_messages: p.hide_messages,
        hide_capture: p.hide_capture,
        capture_supported: CAPTURE_SUPPORTED,
        prompt: s.streamer.prompt.as_ref().map(|q| StreamerPromptView {
            seq: Id(q.seq),
            app: q.app.clone(),
            start: q.start,
        }),
    }
}

/// Streamer mode commands that go to `bonsai-core`; `None` for any other command.
/// `streamer.reveal` and `streamer.show_messages` only change the UI and never reach here.
pub(super) fn run(core: &Core, id: CommandId, inv: &Invocation) -> Option<Result<(), String>> {
    let arg = || inv.arg.clone().unwrap_or_default();
    Some(match id {
        CommandId::ToggleStreamer => {
            let on = match arg().trim().to_ascii_lowercase().as_str() {
                "" => None,
                "on" => Some(true),
                "off" => Some(false),
                other => return Some(Err(format!("\"{other}\" isn't on or off."))),
            };
            core.set_streamer(on);
            Ok(())
        }
        CommandId::StreamerDetect => core.set_streamer_detect(&arg()),
        CommandId::StreamerIgnore => core.ignore_streamer_app(&arg()),
        CommandId::StreamerAskAgain => {
            core.streamer_ask_again();
            Ok(())
        }
        _ => return None,
    })
}
