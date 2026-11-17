//! Streamer mode: the UI hides private things (balances, other accounts, notes, chat previews,
//! notification text) while it is on. A background job looks for recording apps every few
//! seconds and asks to turn it on when one opens, and to turn it off when the last one closes.
//!
//! Detection only reads running process names. Whether OBS is actually live would need its
//! local WebSocket, which the no-network rule forbids, so "detected" means "the app is open".
//! Nothing here writes to Roblox.

use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::alerts::Alert;
use crate::core::{Cmd, Core, Inner};

/// How often running apps are checked.
const EVERY: Duration = Duration::from_secs(5);

/// Linux keeps only the first 15 bytes of a process name (`TASK_COMM_LEN` less the nul), so
/// "streamlabs-desktop" shows up as "streamlabs-desk". Elsewhere names are whole.
const NAME_CUT: Option<usize> = if cfg!(target_os = "linux") { Some(15) } else { None };

/// Recording and streaming apps found by process name, as `(label, names)`. Names are
/// lowercase without `.exe`. Discord is left out: it is open nearly all the time.
pub const KNOWN_APPS: &[(&str, &[&str])] = &[
    (
        "OBS",
        &["obs", "obs64", "obs32", "obs-studio", "com.obsproject.studio"],
    ),
    (
        "Streamlabs",
        &["streamlabs", "streamlabs obs", "streamlabs desktop"],
    ),
    (
        "XSplit",
        &["xsplit.core", "xsplitbroadcaster", "xsplit broadcaster"],
    ),
    ("Twitch Studio", &["twitch studio", "twitchstudio"]),
    ("Medal", &["medal", "medalencoder"]),
    ("NVIDIA ShadowPlay", &["nvsphelper64", "nvsphelper"]),
];

/// What a recording app opening does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Detect {
    /// Offer to turn streamer mode on (the default).
    Ask,
    /// Turn it on. Allowed because it only changes what the UI shows.
    Auto,
    /// Don't look for recording apps.
    Off,
}

impl Detect {
    pub const CHOICES: &'static [(&'static str, &'static str)] =
        &[("ask", "Ask"), ("auto", "Turn on automatically"), ("off", "Off")];
    pub fn parse(s: &str) -> Option<Detect> {
        match s.trim().to_ascii_lowercase().as_str() {
            "ask" => Some(Detect::Ask),
            "auto" | "automatic" => Some(Detect::Auto),
            "off" => Some(Detect::Off),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Detect::Ask => "ask",
            Detect::Auto => "auto",
            Detect::Off => "off",
        }
    }
}

/// Saved streamer mode settings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StreamerPrefs {
    /// Streamer mode is on. Kept across restarts so a restart mid-stream stays hidden.
    pub on: bool,
    /// `ask`, `auto` or `off`; see [`Detect`].
    pub detect: String,
    /// More app names to look for, separated by commas.
    pub apps: String,
    /// Apps the user said not to ask about, by label.
    pub ignored: Vec<String>,
    /// The Messages screen stays hidden until unhidden.
    pub hide_messages: bool,
    /// The window is blank in screen capture where the OS allows it.
    pub hide_capture: bool,
}

impl Default for StreamerPrefs {
    fn default() -> Self {
        Self {
            on: false,
            detect: "ask".into(),
            apps: String::new(),
            ignored: Vec::new(),
            hide_messages: false,
            hide_capture: false,
        }
    }
}

impl StreamerPrefs {
    pub fn detect(&self) -> Detect {
        Detect::parse(&self.detect).unwrap_or(Detect::Ask)
    }
    fn ignores(&self, app: &str) -> bool {
        self.ignored.iter().any(|i| i.eq_ignore_ascii_case(app))
    }
}

/// A process name as compared: trimmed, lowercase, without `.exe`.
pub fn normalize(name: &str) -> String {
    let n = name.trim().to_lowercase();
    n.strip_suffix(".exe").map(str::to_owned).unwrap_or(n)
}

/// `name` without a trailing `.exe` in any case, for showing.
fn without_exe(name: &str) -> &str {
    let cut = name.len().saturating_sub(4);
    match name.get(cut..) {
        Some(ext) if ext.eq_ignore_ascii_case(".exe") => &name[..cut],
        _ => name,
    }
}

/// Whether the normalized process name `seen` is the app name `want`. With `cut`, a name of
/// exactly that many bytes also matches the start of a longer `want`, since the OS cut it.
fn same_app(seen: &str, want: &str, cut: Option<usize>) -> bool {
    seen == want
        || cut.is_some_and(|n| {
            seen.len() == n && want.len() > n && want.as_bytes().starts_with(seen.as_bytes())
        })
}

/// The user's extra app names from `StreamerPrefs::apps`, as `(label, name)`.
pub fn extra_apps(text: &str) -> Vec<(String, String)> {
    text.split([',', '\n'])
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(|t| (without_exe(t).to_owned(), normalize(t)))
        .collect()
}

/// The labels of recording apps among `processes`, sorted and without repeats.
pub fn detect<'a>(processes: impl IntoIterator<Item = &'a str>, extra: &[(String, String)]) -> Vec<String> {
    detect_cut(processes, extra, NAME_CUT)
}

fn detect_cut<'a>(
    processes: impl IntoIterator<Item = &'a str>,
    extra: &[(String, String)],
    cut: Option<usize>,
) -> Vec<String> {
    let mut found = Vec::new();
    for p in processes {
        let n = normalize(p);
        let hit = KNOWN_APPS
            .iter()
            .find(|(_, names)| names.iter().any(|w| same_app(&n, w, cut)))
            .map(|(label, _)| (*label).to_owned())
            .or_else(|| {
                extra
                    .iter()
                    .find(|(_, x)| same_app(&n, x, cut))
                    .map(|(l, _)| l.clone())
            });
        if let Some(label) = hit
            && !found.contains(&label)
        {
            found.push(label);
        }
    }
    found.sort();
    found
}

/// What one check decided.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// Ask to turn streamer mode on for this app.
    AskOn(String),
    /// Turn it on for this app.
    TurnOn(String),
    /// The last app closed while streamer mode is on: offer to turn it off. It never turns
    /// off on its own.
    AskOff(String),
}

/// Compares the apps seen last time with now. Ignored apps count as not running.
pub fn step(prev: &[String], now: &[String], prefs: &StreamerPrefs) -> Option<Step> {
    let detect = prefs.detect();
    if detect == Detect::Off {
        return None;
    }
    let seen = |v: &[String]| {
        v.iter()
            .filter(|a| !prefs.ignores(a))
            .cloned()
            .collect::<Vec<_>>()
    };
    let (prev, now) = (seen(prev), seen(now));
    if let Some(app) = now.iter().find(|a| !prev.contains(a)) {
        if prefs.on {
            return None;
        }
        return Some(match detect {
            Detect::Auto => Step::TurnOn(app.clone()),
            _ => Step::AskOn(app.clone()),
        });
    }
    if now.is_empty()
        && prefs.on
        && let Some(app) = prev.first()
    {
        return Some(Step::AskOff(app.clone()));
    }
    None
}

/// The question on screen: turn streamer mode on (`start`) or off for `app`. The UI shows it
/// once per `seq`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Prompt {
    pub seq: u64,
    pub app: String,
    pub start: bool,
}

impl Prompt {
    /// Whether the question still makes sense with `now` running and streamer mode `on`:
    /// "turn on?" while off and the app is open, "turn off?" while on and it is closed.
    pub fn still_fits(&self, now: &[String], on: bool) -> bool {
        let open = now.iter().any(|a| a.eq_ignore_ascii_case(&self.app));
        if self.start { !on && open } else { on && !open }
    }
}

/// What the recording-app check knows this run.
#[derive(Clone, Debug, Default)]
pub struct StreamerState {
    /// Recording apps open now, by label.
    pub running: Vec<String>,
    pub prompt: Option<Prompt>,
}

/// A desktop notification with the words taken out, for while streamer mode is on.
pub fn redact(a: &Alert) -> Alert {
    let body = match a.kind {
        "chats" => "New message",
        "trades" => "Trade update",
        "friend_requests" => "New friend request",
        "friend_joins" => "A friend joined a game",
        "experience_updates" => "An experience was updated",
        "sales" => "New sale",
        _ => "New notification",
    };
    Alert {
        account: a.account,
        kind: a.kind,
        title: "Bonsai".into(),
        body: body.into(),
    }
}

/// Checks running apps every few seconds while detection is on.
pub(crate) async fn watch_loop(inner: Arc<Inner>) {
    // Keep `stat` files closed on Linux; one check every few seconds doesn't need them.
    sysinfo::set_open_files_limit(0);
    let mut sys = Some(sysinfo::System::new());
    let mut tick = tokio::time::interval(EVERY);
    // After sleep or a stall, check once instead of catching up on every missed tick.
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        tick.tick().await;
        let prefs = inner.settings.read().streamer.clone();
        if prefs.detect() == Detect::Off {
            let mut s = inner.store.write();
            if !s.streamer.running.is_empty() || s.streamer.prompt.is_some() {
                s.streamer = StreamerState::default();
                drop(s);
                inner.repaint();
            }
            continue;
        }
        let Some(mut system) = sys.take() else { continue };
        let extra = extra_apps(&prefs.apps);
        let scan = inner.rt.spawn_blocking(move || {
            system.refresh_processes_specifics(
                sysinfo::ProcessesToUpdate::All,
                sysinfo::ProcessRefreshKind::new(),
            );
            // Linux lists threads as processes too, named by the app that made them; only
            // real processes count.
            let names: Vec<String> = system
                .processes()
                .values()
                .filter(|p| p.thread_kind().is_none())
                .map(|p| p.name().to_string_lossy().into_owned())
                .collect();
            let found = detect(names.iter().map(String::as_str), &extra);
            (system, found)
        });
        let Ok((system, now)) = scan.await else {
            sys = Some(sysinfo::System::new());
            continue;
        };
        sys = Some(system);
        inner.clone().note_recording_apps(now).await;
    }
}

impl Inner {
    async fn note_recording_apps(self: Arc<Self>, now: Vec<String>) {
        let prefs = self.settings.read().streamer.clone();
        let decided = {
            let mut s = self.store.write();
            if s.streamer.running == now {
                return;
            }
            let decided = step(&s.streamer.running, &now, &prefs);
            s.streamer.running = now;
            if let Some(Step::AskOn(app) | Step::AskOff(app)) = &decided {
                let seq = s.streamer.prompt.as_ref().map_or(0, |p| p.seq) + 1;
                s.streamer.prompt = Some(Prompt {
                    seq,
                    app: app.clone(),
                    start: matches!(decided, Some(Step::AskOn(_))),
                });
            } else if s
                .streamer
                .prompt
                .as_ref()
                .is_some_and(|p| !p.still_fits(&s.streamer.running, prefs.on))
            {
                // The app closed before "turn on?" was answered, or reopened before "turn
                // off?" was: the question is out of date.
                s.streamer.prompt = None;
            }
            decided
        };
        match decided {
            Some(Step::TurnOn(app)) => {
                // Same as `update_settings`, from inside the loop that handles `SetSettings`.
                self.settings.write().streamer.on = true;
                self.store.write().streamer.prompt = None;
                self.toast(None, format!("{app} is running. Streamer mode is on."), false);
                self.clone().handle(Cmd::SetSettings).await;
            }
            _ => self.repaint(),
        }
    }
}

impl Core {
    /// Turns streamer mode on or off; `None` flips it.
    pub fn set_streamer(&self, on: Option<bool>) {
        let on = on.unwrap_or(!self.inner.settings.read().streamer.on);
        self.update_settings(|s| s.streamer.on = on);
        self.inner.store.write().streamer.prompt = None;
        self.notice(
            if on {
                "Streamer mode is on."
            } else {
                "Streamer mode is off."
            },
            false,
        );
    }

    /// Sets what a recording app opening does: `ask`, `auto` or `off`.
    pub fn set_streamer_detect(&self, v: &str) -> Result<(), String> {
        let d = Detect::parse(v).ok_or_else(|| format!("\"{v}\" isn't ask, auto or off."))?;
        self.update_settings(|s| d.name().clone_into(&mut s.streamer.detect));
        self.inner.repaint();
        Ok(())
    }

    /// Stops asking about `app` when it opens or closes.
    pub fn ignore_streamer_app(&self, app: &str) -> Result<(), String> {
        let app = app.trim();
        if app.is_empty() {
            return Err("Which app?".into());
        }
        self.update_settings(|s| {
            if !s.streamer.ignores(app) {
                s.streamer.ignored.push(app.to_owned());
            }
        });
        self.inner.store.write().streamer.prompt = None;
        self.notice(format!("Won't ask about {app} again."), false);
        Ok(())
    }

    /// Asks about every recording app again.
    pub fn streamer_ask_again(&self) {
        self.update_settings(|s| s.streamer.ignored.clear());
        self.notice("Will ask about every recording app again.", false);
    }

    /// Streamer mode is on and the window should be blank in screen capture.
    pub fn hide_from_capture(&self) -> bool {
        let s = self.inner.settings.read();
        s.streamer.on && s.streamer.hide_capture
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AccountId;

    fn prefs(detect: &str, on: bool) -> StreamerPrefs {
        StreamerPrefs {
            on,
            detect: detect.into(),
            ..StreamerPrefs::default()
        }
    }

    fn v(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn normalizes_names() {
        assert_eq!(normalize(" OBS64.EXE "), "obs64");
        assert_eq!(normalize("Streamlabs OBS.exe"), "streamlabs obs");
        assert_eq!(normalize("obs"), "obs");
    }

    #[test]
    fn detects_known_and_extra_apps_once() {
        let extra = extra_apps("MyRecorder.exe, , capture-tool\n");
        assert_eq!(
            extra,
            vec![
                ("MyRecorder".to_owned(), "myrecorder".to_owned()),
                ("capture-tool".to_owned(), "capture-tool".to_owned())
            ]
        );
        let found = detect(
            [
                "obs64.exe",
                "OBS",
                "Discord.exe",
                "myrecorder.exe",
                "Medal.exe",
                "bash",
            ],
            &extra,
        );
        assert_eq!(found, v(&["Medal", "MyRecorder", "OBS"]));
        assert!(detect(["discord", "Discord.exe", "steam"], &[]).is_empty());
    }

    #[test]
    fn asks_when_an_app_opens() {
        let p = prefs("ask", false);
        assert_eq!(step(&[], &v(&["OBS"]), &p), Some(Step::AskOn("OBS".into())));
        // Still open: nothing new.
        assert_eq!(step(&v(&["OBS"]), &v(&["OBS"]), &p), None);
        // Already on: nothing to ask.
        assert_eq!(step(&[], &v(&["OBS"]), &prefs("ask", true)), None);
    }

    #[test]
    fn auto_turns_on_and_off_does_nothing() {
        assert_eq!(
            step(&[], &v(&["Medal"]), &prefs("auto", false)),
            Some(Step::TurnOn("Medal".into()))
        );
        assert_eq!(step(&[], &v(&["OBS"]), &prefs("off", false)), None);
        assert_eq!(step(&v(&["OBS"]), &[], &prefs("off", true)), None);
    }

    #[test]
    fn offers_off_only_when_the_last_app_closes() {
        let p = prefs("auto", true);
        assert_eq!(step(&v(&["OBS"]), &[], &p), Some(Step::AskOff("OBS".into())));
        assert_eq!(step(&v(&["Medal", "OBS"]), &v(&["OBS"]), &p), None);
        // Off already: nothing to offer.
        assert_eq!(step(&v(&["OBS"]), &[], &prefs("ask", false)), None);
    }

    #[test]
    fn ignored_apps_never_ask() {
        let mut p = prefs("ask", false);
        p.ignored = v(&["obs"]);
        assert_eq!(step(&[], &v(&["OBS"]), &p), None);
        assert_eq!(
            step(&v(&["OBS"]), &v(&["OBS", "Medal"]), &p),
            Some(Step::AskOn("Medal".into()))
        );
        p.on = true;
        assert_eq!(step(&v(&["OBS"]), &[], &p), None);
    }

    #[test]
    fn matches_names_linux_cut_short() {
        let extra = extra_apps("streamlabs-desktop, Recorder.EXE");
        assert_eq!(extra[1].0, "Recorder");
        // Linux shows the first 15 bytes.
        assert_eq!(
            detect_cut(["streamlabs-desk", "com.obsproject."], &extra, Some(15)),
            v(&["OBS", "streamlabs-desktop"])
        );
        // A shorter name is not a cut one, and other systems never cut.
        assert!(detect_cut(["streamlabs-de"], &extra, Some(15)).is_empty());
        assert!(detect_cut(["streamlabs-desk"], &extra, None).is_empty());
    }

    #[test]
    fn prompts_go_stale() {
        let ask_on = Prompt {
            seq: 1,
            app: "OBS".into(),
            start: true,
        };
        assert!(ask_on.still_fits(&v(&["OBS"]), false));
        assert!(!ask_on.still_fits(&[], false), "OBS closed before the answer");
        assert!(!ask_on.still_fits(&v(&["OBS"]), true), "already on");
        let ask_off = Prompt {
            start: false,
            ..ask_on
        };
        assert!(ask_off.still_fits(&[], true));
        assert!(!ask_off.still_fits(&v(&["OBS"]), true), "OBS reopened");
        assert!(!ask_off.still_fits(&[], false), "already off");
    }

    #[test]
    fn detect_parses_and_defaults_to_ask() {
        assert_eq!(Detect::parse("Automatic"), Some(Detect::Auto));
        assert_eq!(prefs("nonsense", false).detect(), Detect::Ask);
        for (value, _) in Detect::CHOICES {
            assert_eq!(Detect::parse(value).map(Detect::name), Some(*value));
        }
    }

    #[test]
    fn redacted_alerts_keep_only_the_kind() {
        let a = Alert {
            account: AccountId(7),
            kind: "chats",
            title: "Builderman".into(),
            body: "my address is…".into(),
        };
        let r = redact(&a);
        assert_eq!((r.title.as_str(), r.body.as_str()), ("Bonsai", "New message"));
        assert_eq!((r.account, r.kind), (AccountId(7), "chats"));
    }
}
