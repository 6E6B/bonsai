//! Following a launch until the player closes, closing the player when the user asks, and
//! telling the user what happened. See [`crate::launcher::session`] for the states.
//!
//! Each launch gets a tracker thread that reads the player's new log files and checks its
//! process about once a second. It only reads local files and process lists, and it never
//! acts for the user: it updates [`StoreData::launch`] and shows toasts. Closing Roblox happens
//! only from a user's command ("Close Roblox", or a join they confirmed would close it).

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bonsai_api::AccountId;

use crate::cache::Kind;
use crate::core::{Cmd, Inner};
use crate::launcher::log::LogTail;
use crate::launcher::session::{LaunchSession, Notice, Phase, Seen, disconnect_reason};
use crate::launcher::{ChildProcess, JoinTarget, Launched, Opener, PlayerKind};
use crate::store::{Presence, PresenceKind, ToastAction};

const TICK: Duration = Duration::from_millis(500);
/// Nothing at all seen of the player after this means it didn't start.
const START_TIMEOUT: Duration = Duration::from_secs(45);
/// A bootstrapper may download an update first.
const UPDATE_TIMEOUT: Duration = Duration::from_secs(5 * 60);
/// At most this many new log files followed per launch (a launcher log, then the player's).
const MAX_LOGS: usize = 4;
/// How long a polite close may take before the player is killed.
const CLOSE_GRACE: Duration = Duration::from_secs(8);

pub(crate) fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Which logs and processes were there before the launch, so only new ones are followed.
#[derive(Default)]
struct Before {
    logs: HashSet<PathBuf>,
    pids: HashSet<u32>,
}

fn snapshot(opener: &dyn Opener) -> Before {
    Before {
        logs: opener
            .log_dirs()
            .iter()
            .flat_map(|d| d.logs())
            .map(|(p, _)| p)
            .collect(),
        pids: opener.players().iter().map(|p| p.pid).collect(),
    }
}

impl crate::Core {
    /// A Roblox player runs, or a launch is on its way.
    pub fn roblox_open(&self) -> bool {
        let s = self.inner.store.read();
        s.player_open || s.launch.as_ref().is_some_and(|l| l.live())
    }
}

impl Inner {
    /// Starts a session for a launch, unless the same one is already being prepared.
    /// Returns its seq.
    pub(crate) fn begin_launch(&self, account: AccountId, target: &JoinTarget) -> Option<u64> {
        let mut s = self.store.write();
        if let Some(l) = &s.launch
            && l.live()
            && l.phase == Phase::Preparing
            && l.account == account
            && l.target == *target
        {
            return None;
        }
        let seq = s.launch.as_ref().map_or(1, |l| l.seq + 1);
        s.launch = Some(LaunchSession::new(seq, account, target.clone(), now_ms()));
        drop(s);
        self.repaint();
        Some(seq)
    }

    /// Runs `f` on session `seq` if it is still the current one, and repaints.
    fn with_session<R>(&self, seq: u64, f: impl FnOnce(&mut LaunchSession) -> R) -> Option<R> {
        let mut s = self.store.write();
        let l = s.launch.as_mut().filter(|l| l.seq == seq)?;
        let r = f(l);
        drop(s);
        self.repaint();
        Some(r)
    }

    pub(crate) fn fail_launch(&self, seq: u64) {
        self.with_session(seq, |l| l.end(Phase::Failed, now_ms()));
    }

    /// Closes every running player: politely, then by force. True when none is left.
    pub(crate) async fn close_players(self: &Arc<Self>) -> bool {
        let opener = self.opener();
        let games = |o: &Arc<dyn Opener>| -> Vec<u32> {
            o.players()
                .into_iter()
                .filter(|p| p.is_game())
                .map(|p| p.pid)
                .collect()
        };
        let o = opener.clone();
        let mut left = blocking(move || games(&o)).await;
        for force in [false, true] {
            if left.is_empty() {
                return true;
            }
            for pid in &left {
                if let Err(e) = opener.close(*pid, force) {
                    tracing::warn!("closing the player: {e}");
                }
            }
            let deadline = Instant::now()
                + if force {
                    Duration::from_secs(3)
                } else {
                    CLOSE_GRACE
                };
            loop {
                tokio::time::sleep(Duration::from_millis(250)).await;
                let o = opener.clone();
                left = blocking(move || games(&o)).await;
                if left.is_empty() || Instant::now() >= deadline {
                    break;
                }
            }
        }
        let gone = left.is_empty();
        self.set_player_open(!gone);
        gone
    }

    /// Hands the URI to the player and starts following session `seq`.
    pub(crate) async fn open_player(
        self: &Arc<Self>,
        seq: u64,
        uri: &bonsai_api::Secret,
    ) -> Result<(), String> {
        let opener = self.opener();
        let o = opener.clone();
        let before = blocking(move || snapshot(&*o)).await;
        let launched = opener.launch(uri)?;
        self.with_session(seq, |l| l.opened(now_ms()));
        let child = match launched {
            Launched::Handoff => None,
            Launched::Child(c) => Some(c),
        };
        let inner = self.clone();
        let spawned = std::thread::Builder::new()
            .name("bonsai-launch".into())
            .spawn(move || inner.track(seq, opener, child, before));
        if let Err(e) = spawned {
            tracing::warn!("launch tracker: {e}");
        }
        Ok(())
    }

    /// The tracker loop for session `seq`. Ends when the session does or another replaces it.
    fn track(
        self: Arc<Self>,
        seq: u64,
        opener: Arc<dyn Opener>,
        mut child: Option<Box<dyn ChildProcess>>,
        before: Before,
    ) {
        let start = Instant::now();
        let mut tails: Vec<LogTail> = Vec::new();
        let mut tick: u64 = 0;
        let mut bootstrapper = false;
        // The child's exit code, kept only when it ran the game itself.
        let mut child_code: Option<Option<i32>> = None;
        let mut child_failed: Option<(Option<i32>, u64)> = None;
        let mut missing = 0u32;
        loop {
            std::thread::sleep(TICK);
            tick += 1;
            let Some(s) = self.session(seq).filter(LaunchSession::live) else {
                break;
            };
            let mut seen = Vec::new();

            // New log files, then what was appended to them.
            if tails.is_empty() || tick.is_multiple_of(4) {
                for d in opener.log_dirs() {
                    for (p, _) in d.logs() {
                        if tails.len() < MAX_LOGS
                            && !before.logs.contains(&p)
                            && !tails.iter().any(|t| t.path == p)
                        {
                            tails.push(LogTail::new(p));
                        }
                    }
                }
            }
            for t in &mut tails {
                if let Ok(evs) = t.read_new() {
                    seen.extend(evs.into_iter().map(Seen::Log));
                }
            }

            if let Some(c) = child.as_mut()
                && let Some(code) = c.exited()
            {
                child = None;
                if s.process_seen || s.log_seen {
                    child_code = Some(code);
                } else if code != Some(0) {
                    // Wait a tick for a log that explains it (Sober already running).
                    child_failed = Some((code, tick));
                }
            }

            if tick % 2 == 1 {
                let ps = opener.players();
                bootstrapper |= ps.iter().any(|p| p.kind == PlayerKind::Bootstrapper);
                let games: Vec<_> = ps.iter().filter(|p| p.is_game()).collect();
                self.set_player_open(!games.is_empty());
                let ours = games
                    .iter()
                    .find(|p| !before.pids.contains(&p.pid))
                    .or(games.first());
                match ours {
                    Some(p) => {
                        missing = 0;
                        if s.pid != Some(p.pid) {
                            seen.push(Seen::Process(p.pid));
                        }
                    }
                    None if s.process_seen && child.is_none() => missing += 1,
                    None => {}
                }
            }

            let ev_count = seen.len();
            let mut notices = Vec::new();
            let mut snap = None;
            self.with_session(seq, |l| {
                let now = now_ms();
                for x in seen {
                    notices.extend(l.apply(x, now));
                }
                if let Some((code, at)) = child_failed
                    && tick > at + 1
                    && !l.process_seen
                {
                    notices.extend(l.apply(
                        if l.log_seen {
                            Seen::Exited(code)
                        } else {
                            Seen::CommandFailed(code)
                        },
                        now,
                    ));
                }
                // Gone for two checks in a row: the player closed.
                let gone = missing >= 2
                    // No process to watch (not found by name), but the command that ran the
                    // game has exited.
                    || (!l.process_seen && child.is_none() && child_code.is_some());
                if gone {
                    notices.extend(l.apply(Seen::Exited(child_code.flatten()), now));
                }
                let waited = start.elapsed();
                let limit = if bootstrapper {
                    UPDATE_TIMEOUT
                } else {
                    START_TIMEOUT
                };
                if waited >= limit && child.is_none() && l.phase == Phase::Opening {
                    // Without a log folder this player may just be one Bonsai can't see.
                    let known = bootstrapper || opener.log_dirs().iter().any(|d| d.dir.is_dir());
                    notices.extend(l.apply(if known { Seen::TimedOut } else { Seen::Untracked }, now));
                }
                snap = Some(l.clone());
            });
            let Some(snap) = snap else { break };
            for n in notices {
                self.launch_notice(&snap, n);
            }
            if !snap.live() {
                if snap.process_seen || ev_count > 0 {
                    self.set_player_open(false);
                }
                break;
            }
        }
        if let Some(mut c) = child {
            // Reap it whenever it exits.
            std::thread::spawn(move || {
                while c.exited().is_none() {
                    std::thread::sleep(Duration::from_secs(1));
                }
            });
        }
    }

    fn session(&self, seq: u64) -> Option<LaunchSession> {
        self.store.read().launch.clone().filter(|l| l.seq == seq)
    }

    pub(crate) fn set_player_open(&self, open: bool) {
        let mut s = self.store.write();
        if s.player_open != open {
            s.player_open = open;
            drop(s);
            self.repaint();
        }
    }

    fn game_name(&self, l: &LaunchSession) -> Option<String> {
        let s = self.store.read();
        l.universe_id
            .and_then(|u| s.games.get(&u))
            .or_else(|| l.place_id.and_then(|p| s.game_for_place(p)))
            .map(|g| g.name.clone())
            .filter(|n| !n.is_empty())
    }

    fn launch_notice(self: &Arc<Self>, l: &LaunchSession, n: Notice) {
        let rejoin = || ToastAction {
            label: "Rejoin".into(),
            command: "game.rejoin".into(),
            arg: None,
        };
        let settings = || ToastAction {
            label: "Launch settings".into(),
            command: "nav.settings".into(),
            arg: Some("general".into()),
        };
        let account = Some(l.account);
        match n {
            Notice::Connected => self.note_joined(l),
            Notice::Place(p) => {
                let known = self.store.read().place_universe.contains_key(&p);
                if !known {
                    let inner = self.clone();
                    let account = l.account;
                    self.rt.spawn(async move {
                        inner.handle(Cmd::LoadPlace { account, place_id: p }).await;
                    });
                }
            }
            Notice::Disconnected(code) => {
                let reason = disconnect_reason(code.into());
                let text = match self.game_name(l) {
                    Some(g) => format!("Disconnected from {g}: {reason} ({code})"),
                    None => format!("Disconnected: {reason} ({code})"),
                };
                self.toast_with(account, text, true, Some(rejoin()));
            }
            Notice::Ended(Phase::Crashed) => {
                let text = match self.game_name(l) {
                    Some(g) => format!("Roblox closed unexpectedly in {g}"),
                    None => "Roblox closed unexpectedly".into(),
                };
                self.toast_with(account, text, true, Some(rejoin()));
            }
            Notice::Ended(Phase::AlreadyRunning) => self.toast_with(
                account,
                "Sober was already open, so it didn't join.",
                true,
                Some(ToastAction {
                    label: "Close and join".into(),
                    command: "game.rejoin".into(),
                    arg: None,
                }),
            ),
            Notice::Ended(Phase::DidNotStart) => self.toast_with(
                account,
                "Roblox didn't open. Check how Bonsai launches it.",
                true,
                Some(settings()),
            ),
            Notice::Ended(Phase::CommandFailed) => {
                let text = match l.code {
                    Some(c) => format!("The launch command failed (exit code {c})."),
                    None => "The launch command failed.".into(),
                };
                self.toast_with(account, text, true, Some(settings()));
            }
            Notice::Ended(_) => {}
        }
    }

    /// Connected to a server: remember it in recent servers, and the experience in recents.
    fn note_joined(self: &Arc<Self>, l: &LaunchSession) {
        let (Some(place_id), Some(job_id)) = (l.place_id, l.job_id.clone()) else {
            return;
        };
        let universe = {
            let mut s = self.store.write();
            if let Some(u) = l.universe_id {
                s.place_universe.entry(place_id).or_insert(u);
            }
            s.place_universe.get(&place_id).copied()
        };
        let root = universe.and_then(|u| self.store.read().games.get(&u).map(|g| g.root_place_id));
        self.note_own_presence(
            l.account,
            &Presence {
                kind: PresenceKind::InGame,
                place_id: Some(place_id),
                root_place_id: root,
                universe_id: universe,
                job_id: Some(job_id),
                last_location: String::new(),
            },
        );
        if let Some(u) = universe {
            let recent = {
                let mut s = self.store.write();
                let d = s.data.entry(l.account).or_default();
                if d.recent.first() == Some(&u) {
                    None
                } else {
                    d.recent.retain(|x| *x != u);
                    d.recent.insert(0, u);
                    d.recent.truncate(30);
                    Some(d.recent.clone())
                }
            };
            if let Some(r) = recent {
                self.cache.put(Kind::Recent, l.account, "recent", &r);
            }
            let missing = !self.store.read().games.contains_key(&u);
            if missing {
                let inner = self.clone();
                self.rt
                    .spawn(async move { inner.handle(Cmd::LoadUniverses(vec![u])).await });
            }
        }
    }

    /// "Close Roblox".
    pub(crate) async fn close_game(self: &Arc<Self>) {
        let seq = self
            .store
            .read()
            .launch
            .as_ref()
            .filter(|l| l.live())
            .map(|l| l.seq);
        if let Some(seq) = seq {
            self.with_session(seq, |l| l.apply(Seen::Closing, now_ms()));
        }
        let opener = self.opener();
        let any = blocking(move || opener.players().iter().any(|p| p.is_game())).await;
        if !any {
            if let Some(seq) = seq {
                // Nothing on screen yet: stop waiting for it.
                self.with_session(seq, |l| l.apply(Seen::Exited(None), now_ms()));
            } else {
                self.toast(None, "Roblox isn't open.", false);
            }
            return;
        }
        if !self.close_players().await {
            self.toast(None, "Couldn't close Roblox.", true);
        }
    }

    /// "Rejoin": the last session's server, as the account that played it.
    pub(crate) async fn rejoin(self: &Arc<Self>) {
        let last = self
            .store
            .read()
            .launch
            .as_ref()
            .map(|l| (l.account, l.rejoin_target()));
        match last {
            Some((account, target)) => self.launch(account, target).await,
            None => self.toast(None, "Nothing to rejoin yet.", false),
        }
    }
}

/// Watches for a player Bonsai didn't start, so joining can warn that it will close it.
pub(crate) async fn player_watch_loop(inner: Arc<Inner>) {
    loop {
        let tracked = inner.store.read().launch.as_ref().is_some_and(|l| l.live());
        if !tracked {
            let opener = inner.opener();
            let open = blocking(move || opener.players().iter().any(|p| p.is_game())).await;
            inner.set_player_open(open);
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

/// Runs blocking work (process lists, file reads) off the async threads. A panic gives the
/// default, which reads as "nothing found".
async fn blocking<T: Default + Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    tokio::task::spawn_blocking(f).await.unwrap_or_default()
}
