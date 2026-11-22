//! One launch, from the click to the player closing. A pure state machine: the tracker feeds
//! it log events and what it sees of the player's process, and it says what changed.

use bonsai_api::AccountId;

use super::JoinTarget;
use super::log::{LEFT, LogEvent};

/// Where a launch is. The first group is while the player runs; the rest are how it ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// Getting a ticket, and closing a player that was already open.
    Preparing,
    /// Handed to the player; nothing seen of it yet.
    Opening,
    /// The player process or its log appeared.
    Starting,
    /// Joining a server.
    Joining,
    InGame,
    Teleporting,
    /// In the player's own menu, out of any experience.
    Menu,
    /// The server dropped or kicked the player. While the player is open it shows the reason;
    /// after it closes, this is how the session ended.
    Disconnected,
    /// The player closed normally.
    Closed,
    /// The player closed without leaving or logging a clean exit.
    Crashed,
    /// Sober was already running and refused to start another copy.
    AlreadyRunning,
    /// Nothing started.
    DidNotStart,
    /// The custom launch command failed.
    CommandFailed,
    /// Another launch took its place.
    Replaced,
    /// Bonsai could not launch at all (no ticket, no handler).
    Failed,
    /// Handed off to a player Bonsai can't follow (no log folder, no known process).
    Untracked,
}

/// What the tracker saw.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Seen {
    Log(LogEvent),
    /// A player process is running.
    Process(u32),
    /// The player is gone, with the launch command's exit code when that was what ran.
    Exited(Option<i32>),
    /// The launch command exited with an error before anything else was seen.
    CommandFailed(Option<i32>),
    /// Too long without any sign of the player.
    TimedOut,
    /// Too long without a sign, from a player Bonsai has no way to see.
    Untracked,
    /// The user asked Bonsai to close the player.
    Closing,
    Replaced,
}

/// Something the user should hear about, or Bonsai should act on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notice {
    /// Connected to a server: remember it.
    Connected,
    /// The session's place changed: name it.
    Place(u64),
    /// Disconnected with this code (never [`LEFT`]).
    Disconnected(u32),
    /// Ended this way, for the phases that warrant telling the user.
    Ended(Phase),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LaunchSession {
    /// Bumped per launch; a tracker for an older one stops.
    pub seq: u64,
    pub account: AccountId,
    pub target: JoinTarget,
    pub phase: Phase,
    /// The disconnect code, or the launch command's exit code.
    pub code: Option<i64>,
    pub place_id: Option<u64>,
    pub universe_id: Option<u64>,
    pub job_id: Option<String>,
    /// Unix ms.
    pub started_at: i64,
    /// When `phase` last changed, unix ms.
    pub phase_at: i64,
    /// When it first connected to a server, unix ms.
    pub joined_at: Option<i64>,
    /// When the player closed or the launch failed, unix ms. `None` while it runs.
    pub ended_at: Option<i64>,
    pub pid: Option<u32>,
    /// The log has Sober's lifecycle lines, so a normal exit is always logged.
    pub lifecycle: bool,
    /// The player is on its way out: it left, logged a clean exit, or Bonsai closed it.
    pub closing: bool,
    pub log_seen: bool,
    pub process_seen: bool,
    /// Between a teleport starting and connecting to the new server. The old server's
    /// disconnect can land after the new join starts.
    pub teleporting: bool,
}

impl LaunchSession {
    pub fn new(seq: u64, account: AccountId, target: JoinTarget, now: i64) -> LaunchSession {
        LaunchSession {
            seq,
            account,
            place_id: target.place_id(),
            target,
            phase: Phase::Preparing,
            code: None,
            universe_id: None,
            job_id: None,
            started_at: now,
            phase_at: now,
            joined_at: None,
            ended_at: None,
            pid: None,
            lifecycle: false,
            closing: false,
            log_seen: false,
            process_seen: false,
            teleporting: false,
        }
    }

    pub fn live(&self) -> bool {
        self.ended_at.is_none()
    }

    /// The player shows something on screen, so it must be closed before another launch.
    pub fn on_screen(&self) -> bool {
        self.live() && !matches!(self.phase, Phase::Preparing | Phase::Opening)
    }

    fn set(&mut self, phase: Phase, now: i64) {
        if self.phase != phase {
            self.phase = phase;
            self.phase_at = now;
        }
    }

    pub fn end(&mut self, phase: Phase, now: i64) {
        self.set(phase, now);
        self.ended_at = Some(now);
    }

    /// The handoff to the player worked.
    pub fn opened(&mut self, now: i64) {
        if self.phase == Phase::Preparing {
            self.set(Phase::Opening, now);
        }
    }

    pub fn apply(&mut self, seen: Seen, now: i64) -> Vec<Notice> {
        let mut out = Vec::new();
        if !self.live() {
            return out;
        }
        match seen {
            Seen::Log(ev) => self.log(ev, now, &mut out),
            Seen::Process(pid) => {
                self.pid = Some(pid);
                self.process_seen = true;
                if self.phase == Phase::Opening {
                    self.set(Phase::Starting, now);
                }
            }
            Seen::Exited(code) => {
                let end = match self.phase {
                    // Kicked, then closed from the error screen: still a disconnect.
                    Phase::Disconnected => Phase::Disconnected,
                    _ if self.closing => Phase::Closed,
                    // A command like `flatpak run` that ran the game itself and exited fine.
                    _ if code == Some(0) => Phase::Closed,
                    Phase::Joining | Phase::InGame | Phase::Teleporting => Phase::Crashed,
                    // Died while starting, with Sober's lifecycle showing it got that far.
                    Phase::Starting if self.lifecycle => Phase::Crashed,
                    _ => Phase::Closed,
                };
                if end != Phase::Disconnected
                    && let Some(c) = code.filter(|c| *c != 0)
                {
                    self.code = Some(c.into());
                }
                self.end(end, now);
                if end == Phase::Crashed {
                    out.push(Notice::Ended(end));
                }
            }
            Seen::CommandFailed(code) => {
                self.code = code.map(Into::into);
                self.end(Phase::CommandFailed, now);
                out.push(Notice::Ended(Phase::CommandFailed));
            }
            Seen::TimedOut => {
                if self.phase == Phase::Opening {
                    self.end(Phase::DidNotStart, now);
                    out.push(Notice::Ended(Phase::DidNotStart));
                }
            }
            Seen::Untracked => {
                if self.phase == Phase::Opening {
                    self.end(Phase::Untracked, now);
                }
            }
            Seen::Closing => self.closing = true,
            Seen::Replaced => self.end(Phase::Replaced, now),
        }
        out
    }

    fn log(&mut self, ev: LogEvent, now: i64, out: &mut Vec<Notice>) {
        self.log_seen = true;
        if self.phase == Phase::Opening {
            self.set(Phase::Starting, now);
        }
        match ev {
            LogEvent::AppReady => self.lifecycle = true,
            LogEvent::JoinStart => {
                self.closing = false;
                if !matches!(self.phase, Phase::Joining | Phase::Teleporting) {
                    self.set(Phase::Joining, now);
                }
            }
            LogEvent::Joining { job_id, place_id } => {
                self.closing = false;
                self.code = None;
                self.job_id = Some(job_id);
                if self.place_id != Some(place_id) {
                    self.place_id = Some(place_id);
                    self.universe_id = None;
                    out.push(Notice::Place(place_id));
                }
                if !self.teleporting {
                    self.set(Phase::Joining, now);
                }
            }
            LogEvent::JoinInfo {
                place_id,
                universe_id,
            } => {
                if place_id.is_none() || place_id == self.place_id {
                    self.universe_id = universe_id.or(self.universe_id);
                }
            }
            LogEvent::Connected => {
                if matches!(self.phase, Phase::Joining | Phase::Starting | Phase::Teleporting) {
                    self.teleporting = false;
                    self.closing = false;
                    self.set(Phase::InGame, now);
                    self.joined_at.get_or_insert(now);
                    out.push(Notice::Connected);
                }
            }
            LogEvent::Teleporting => {
                if matches!(self.phase, Phase::InGame | Phase::Joining | Phase::Teleporting) {
                    self.teleporting = true;
                    self.set(Phase::Teleporting, now);
                }
            }
            LogEvent::Disconnected { code } => {
                // A teleport drops the old server on purpose; a leave or close sends 285.
                if self.teleporting || self.phase == Phase::Disconnected {
                    return;
                }
                if code == LEFT {
                    self.closing = true;
                    return;
                }
                if !matches!(self.phase, Phase::Joining | Phase::InGame | Phase::Starting) {
                    return;
                }
                self.code = Some(code.into());
                self.set(Phase::Disconnected, now);
                out.push(Notice::Disconnected(code));
            }
            LogEvent::Leaving => {
                self.closing = true;
                if self.phase != Phase::Disconnected {
                    self.set(Phase::Menu, now);
                }
            }
            LogEvent::AtMenu => {
                // Stays closing when it came back from a leave; an exit from the menu is
                // normal either way.
                self.set(Phase::Menu, now);
            }
            LogEvent::CleanExit => self.closing = true,
            LogEvent::AlreadyRunning => {
                self.end(Phase::AlreadyRunning, now);
                out.push(Notice::Ended(Phase::AlreadyRunning));
            }
        }
    }

    /// Where "Rejoin" goes: the last server when known, else what was launched.
    pub fn rejoin_target(&self) -> JoinTarget {
        match (&self.target, self.place_id, &self.job_id) {
            // A private server's job can't be joined by id; ask for the private server again.
            (JoinTarget::PrivateServer { .. }, ..) => self.target.clone(),
            (_, Some(place_id), Some(job_id)) => JoinTarget::Server {
                place_id,
                job_id: job_id.clone(),
            },
            (_, Some(place_id), None) => JoinTarget::Place { place_id },
            _ => self.target.clone(),
        }
    }
}

/// Plain words for a Roblox `ConnectionError` code, as the engine's enum names them.
pub fn disconnect_reason(code: i64) -> &'static str {
    match code {
        260..=262 | 296 | 297 | 299 | 300 => "Network error",
        264 | 273 => "The account joined from somewhere else",
        266 | 290 | 298 => "The connection timed out",
        267 => "Kicked by the experience",
        268 => "Kicked for unusual activity",
        271 | 302 => "The server emptied",
        274 => "The experience is under maintenance",
        275 => "Roblox is under maintenance",
        276 => "The server asked to rejoin",
        277 => "Lost connection",
        278 => "Kicked for being idle",
        279 => "Could not connect",
        280 => "Roblox needs an update",
        281 => "Blocked by a security policy",
        282 => "This network is blocked",
        284 => "The player had an error",
        285 => "Left",
        286 => "Removed from the private server",
        287 | 309 => "The experience was moderated",
        288 => "The server shut down",
        291 => "Removed from the server",
        292 => "Ran out of memory",
        301 => "The server restarted for an update",
        303 => "The player froze",
        307 => "Screen time limit reached",
        515 => "The experience is unavailable",
        517 | 771 => "The server closed",
        518 | 772 => "The server is full",
        523 => "The experience is restricted",
        524 | 773 => "Not allowed to join",
        525 | 774 => "Too many join attempts",
        529 => "Could not reach Roblox",
        533 => "Blocked by privacy settings",
        541 => "The private server's owner isn't in it",
        542 => "Age check required",
        543 => "Parent approval required",
        600 => "Banned from this experience",
        601 => "This device isn't allowed",
        769 | 770 | 775 => "The teleport failed",
        _ => "Disconnected",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ME: AccountId = AccountId(1);
    const JOB: &str = "11111111-2222-4333-8444-555555555555";

    fn opened() -> LaunchSession {
        let mut s = LaunchSession::new(1, ME, JoinTarget::Place { place_id: 10 }, 0);
        s.opened(1);
        s
    }

    fn run(s: &mut LaunchSession, seen: impl IntoIterator<Item = Seen>) -> Vec<Notice> {
        let mut out = Vec::new();
        for (i, x) in seen.into_iter().enumerate() {
            out.extend(s.apply(x, 10 + i as i64));
        }
        out
    }

    fn log(e: LogEvent) -> Seen {
        Seen::Log(e)
    }

    fn joining(job: &str, place: u64) -> Seen {
        log(LogEvent::Joining {
            job_id: job.into(),
            place_id: place,
        })
    }

    fn in_game(s: &mut LaunchSession) {
        run(
            s,
            [
                Seen::Process(7),
                log(LogEvent::AppReady),
                log(LogEvent::JoinStart),
                joining(JOB, 10),
                log(LogEvent::Connected),
            ],
        );
        assert_eq!(s.phase, Phase::InGame);
    }

    #[test]
    fn joins_then_leaves_and_closes() {
        let mut s = opened();
        assert_eq!(s.phase, Phase::Opening);
        assert!(!s.on_screen());
        run(&mut s, [Seen::Process(7)]);
        assert_eq!(s.phase, Phase::Starting);
        assert!(s.on_screen());
        let n = run(
            &mut s,
            [
                log(LogEvent::AppReady),
                log(LogEvent::JoinStart),
                joining(JOB, 10),
            ],
        );
        assert_eq!(s.phase, Phase::Joining);
        assert!(n.is_empty(), "same place as launched: {n:?}");
        let n = run(&mut s, [log(LogEvent::Connected)]);
        assert_eq!(n, vec![Notice::Connected]);
        assert!(s.joined_at.is_some());
        let n = run(
            &mut s,
            [
                log(LogEvent::Leaving),
                log(LogEvent::Disconnected { code: LEFT }),
                log(LogEvent::AtMenu),
            ],
        );
        assert!(n.is_empty());
        assert_eq!(s.phase, Phase::Menu);
        assert!(run(&mut s, [log(LogEvent::CleanExit), Seen::Exited(None)]).is_empty());
        assert_eq!(s.phase, Phase::Closed);
        assert!(!s.live());
    }

    #[test]
    fn teleport_disconnects_are_not_kicks() {
        let mut s = opened();
        in_game(&mut s);
        let n = run(
            &mut s,
            [
                log(LogEvent::Teleporting),
                log(LogEvent::Disconnected { code: 277 }),
                log(LogEvent::Teleporting),
                joining("66666666-7777-4888-9999-aaaaaaaaaaaa", 11),
                log(LogEvent::Connected),
            ],
        );
        assert_eq!(n, vec![Notice::Place(11), Notice::Connected]);
        assert_eq!(s.phase, Phase::InGame);
        assert_eq!(s.job_id.as_deref(), Some("66666666-7777-4888-9999-aaaaaaaaaaaa"));
        assert_eq!(s.place_id, Some(11));
    }

    #[test]
    fn teleport_as_logged_by_sober() {
        // Seen in a real log: the old server's 285 arrives after the new join started.
        let mut s = opened();
        in_game(&mut s);
        run(
            &mut s,
            [
                log(LogEvent::Teleporting),
                joining("66666666-7777-4888-9999-aaaaaaaaaaaa", 10),
                log(LogEvent::Disconnected { code: LEFT }),
                log(LogEvent::Disconnected { code: 277 }),
            ],
        );
        assert_eq!(s.phase, Phase::Teleporting);
        run(&mut s, [log(LogEvent::Connected)]);
        assert_eq!(s.phase, Phase::InGame);
        assert!(!s.closing);
        assert_eq!(
            run(&mut s, [Seen::Exited(None)]),
            vec![Notice::Ended(Phase::Crashed)]
        );
    }

    #[test]
    fn kick_then_close_is_a_disconnect() {
        let mut s = opened();
        in_game(&mut s);
        let n = run(&mut s, [log(LogEvent::Disconnected { code: 267 })]);
        assert_eq!(n, vec![Notice::Disconnected(267)]);
        assert_eq!(s.phase, Phase::Disconnected);
        assert!(s.live(), "the error screen is still open");
        // The same kick logged again, then the 285 the player sends while leaving.
        let n = run(
            &mut s,
            [
                log(LogEvent::Disconnected { code: 267 }),
                log(LogEvent::Disconnected { code: LEFT }),
                Seen::Exited(None),
            ],
        );
        assert!(n.is_empty());
        assert_eq!(s.phase, Phase::Disconnected);
        assert_eq!(s.code, Some(267));
        assert!(!s.live());
    }

    #[test]
    fn rejoin_after_kick_from_the_error_screen() {
        let mut s = opened();
        in_game(&mut s);
        run(&mut s, [log(LogEvent::Disconnected { code: 277 })]);
        let n = run(
            &mut s,
            [
                log(LogEvent::JoinStart),
                joining(JOB, 10),
                log(LogEvent::Connected),
            ],
        );
        assert_eq!(n, vec![Notice::Connected]);
        assert_eq!(s.phase, Phase::InGame);
        assert_eq!(s.code, None);
    }

    #[test]
    fn exit_mid_game_without_leaving_is_a_crash() {
        let mut s = opened();
        in_game(&mut s);
        let n = run(&mut s, [Seen::Exited(None)]);
        assert_eq!(n, vec![Notice::Ended(Phase::Crashed)]);
        assert_eq!(s.phase, Phase::Crashed);
    }

    #[test]
    fn closing_the_window_in_game_is_not_a_crash() {
        // Sober logs a clean exit, then the leave; other players at least send 285.
        for last in [
            log(LogEvent::CleanExit),
            log(LogEvent::Disconnected { code: LEFT }),
        ] {
            let mut s = opened();
            in_game(&mut s);
            let n = run(&mut s, [last, Seen::Exited(None)]);
            assert!(n.is_empty());
            assert_eq!(s.phase, Phase::Closed);
        }
        let mut s = opened();
        in_game(&mut s);
        run(&mut s, [Seen::Closing, Seen::Exited(None)]);
        assert_eq!(s.phase, Phase::Closed, "Bonsai closed it");
        let mut s = opened();
        in_game(&mut s);
        run(&mut s, [Seen::Exited(Some(0))]);
        assert_eq!(s.phase, Phase::Closed, "the command exited fine");
    }

    #[test]
    fn sober_already_running() {
        let mut s = opened();
        let n = run(&mut s, [log(LogEvent::AlreadyRunning), Seen::Exited(Some(134))]);
        assert_eq!(n, vec![Notice::Ended(Phase::AlreadyRunning)]);
        assert_eq!(s.phase, Phase::AlreadyRunning);
    }

    #[test]
    fn nothing_starts() {
        let mut s = opened();
        assert_eq!(
            run(&mut s, [Seen::TimedOut]),
            vec![Notice::Ended(Phase::DidNotStart)]
        );
        assert_eq!(s.phase, Phase::DidNotStart);
        let mut s = opened();
        assert!(run(&mut s, [Seen::Untracked]).is_empty());
        assert_eq!(s.phase, Phase::Untracked);
        // A timeout only counts while nothing was seen.
        let mut s = opened();
        run(&mut s, [Seen::Process(3), Seen::TimedOut]);
        assert_eq!(s.phase, Phase::Starting);
    }

    #[test]
    fn command_fails() {
        let mut s = opened();
        let n = run(&mut s, [Seen::CommandFailed(Some(127))]);
        assert_eq!(n, vec![Notice::Ended(Phase::CommandFailed)]);
        assert_eq!(s.code, Some(127));
    }

    #[test]
    fn crash_while_starting_needs_the_lifecycle() {
        let mut s = opened();
        run(
            &mut s,
            [Seen::Process(3), log(LogEvent::AppReady), Seen::Exited(None)],
        );
        assert_eq!(s.phase, Phase::Crashed);
        let mut s = opened();
        run(&mut s, [Seen::Process(3), Seen::Exited(None)]);
        assert_eq!(s.phase, Phase::Closed);
    }

    #[test]
    fn exit_from_the_menu_is_normal() {
        let mut s = opened();
        in_game(&mut s);
        run(&mut s, [log(LogEvent::Leaving), log(LogEvent::AtMenu)]);
        // Picking another game in the player's menu, then quitting from its menu.
        run(
            &mut s,
            [
                log(LogEvent::JoinStart),
                joining(JOB, 12),
                log(LogEvent::Connected),
            ],
        );
        assert!(!s.closing);
        run(
            &mut s,
            [log(LogEvent::Leaving), log(LogEvent::AtMenu), Seen::Exited(None)],
        );
        assert_eq!(s.phase, Phase::Closed);
    }

    #[test]
    fn ended_sessions_ignore_more() {
        let mut s = opened();
        run(&mut s, [Seen::Replaced]);
        assert_eq!(s.phase, Phase::Replaced);
        assert!(run(&mut s, [Seen::Process(1), log(LogEvent::Connected)]).is_empty());
        assert_eq!(s.phase, Phase::Replaced);
    }

    #[test]
    fn universe_follows_the_place() {
        let mut s = opened();
        in_game(&mut s);
        run(
            &mut s,
            [log(LogEvent::JoinInfo {
                place_id: Some(10),
                universe_id: Some(99),
            })],
        );
        assert_eq!(s.universe_id, Some(99));
        run(&mut s, [joining(JOB, 12)]);
        assert_eq!(s.universe_id, None, "another place, so maybe another universe");
    }

    #[test]
    fn rejoin_targets() {
        let mut s = opened();
        assert_eq!(s.rejoin_target(), JoinTarget::Place { place_id: 10 });
        in_game(&mut s);
        assert_eq!(
            s.rejoin_target(),
            JoinTarget::Server {
                place_id: 10,
                job_id: JOB.into()
            }
        );
        let ps = JoinTarget::PrivateServer {
            place_id: 10,
            link_code: None,
            access_code: Some("a".into()),
        };
        let mut s = LaunchSession::new(1, ME, ps.clone(), 0);
        in_game(&mut s);
        assert_eq!(s.rejoin_target(), ps);
        let f = LaunchSession::new(1, ME, JoinTarget::FollowUser { user_id: 5 }, 0);
        assert_eq!(f.rejoin_target(), JoinTarget::FollowUser { user_id: 5 });
    }

    #[test]
    fn reasons() {
        assert_eq!(disconnect_reason(267), "Kicked by the experience");
        assert_eq!(disconnect_reason(273), "The account joined from somewhere else");
        assert_eq!(disconnect_reason(1), "Disconnected");
    }
}
