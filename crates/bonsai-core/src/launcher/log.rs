//! Reading the Roblox player's log to follow a launch.
//!
//! Every player writes a log: `%LOCALAPPDATA%\Roblox\logs` on Windows, `~/Library/Logs/Roblox`
//! on macOS, and Sober's `sober_logs` folder on Linux. They share the engine's `[FLog::…]`
//! lines, which say when the player joins a server, connects, teleports, is disconnected and
//! goes back to the menu. Sober adds its own lifecycle lines around them.
//!
//! These logs hold private things: the join script with the auth ticket, the account's name,
//! server addresses. So this module only matches a short allowlist of lines and returns typed
//! [`LogEvent`]s with the few values Bonsai uses (place, universe, job id, disconnect code).
//! A raw line never leaves [`parse_line`]: not to the store, not to a log, not to the UI.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

/// What one log line says about the session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogEvent {
    /// Sober finished starting up (`lifecycle: enter_main_loop`). Only Sober writes these, so
    /// it also tells the tracker that a clean exit will be logged.
    AppReady,
    /// The player started joining an experience (`launchUGCGameInternal`), from a launch or
    /// from its own menu.
    JoinStart,
    /// `! Joining game '<job>' place <id> at <address>`. The address is not kept.
    Joining { job_id: String, place_id: u64 },
    /// The join report, which may name the universe.
    JoinInfo {
        place_id: Option<u64>,
        universe_id: Option<u64>,
    },
    /// Connected to the server (`serverId: …`).
    Connected,
    /// A teleport to another server or place started.
    Teleporting,
    /// The connection ended, with Roblox's `ConnectionError` code. 285 means the player left.
    Disconnected { code: u32 },
    /// The player chose to leave the experience.
    Leaving,
    /// Back in the player's own menu.
    AtMenu,
    /// The player is closing normally (Sober's `will_do_clean_exit`).
    CleanExit,
    /// Sober refused to start because another copy is running.
    AlreadyRunning,
}

/// The disconnect code the player sends when the user leaves (`DisconnectClientRequest`).
pub const LEFT: u32 = 285;

/// The event a line means, if it is one of the few Bonsai reads.
pub fn parse_line(line: &str) -> Option<LogEvent> {
    if let Some(i) = line.find("[FLog::").or_else(|| line.find("[DFLog::")) {
        let tagged = &line[i..];
        let end = tagged.find(']')?;
        let tag = tagged[..end].split("::").nth(1)?;
        let rest = tagged[end + 1..].trim_start();
        return parse_flog(tag, rest);
    }
    if line.contains("lifecycle: enter_main_loop") {
        return Some(LogEvent::AppReady);
    }
    if line.contains("lifecycle: will_do_clean_exit") {
        return Some(LogEvent::CleanExit);
    }
    if line.starts_with("FATAL") && line.contains("instance of Sober is already running") {
        return Some(LogEvent::AlreadyRunning);
    }
    None
}

fn parse_flog(tag: &str, rest: &str) -> Option<LogEvent> {
    match tag {
        "Output" => {
            let r = rest.strip_prefix("! Joining game '")?;
            let (job, r) = r.split_once('\'')?;
            let place = r.trim_start().strip_prefix("place ")?;
            let place_id = leading_number(place)?;
            is_job_id(job).then(|| LogEvent::Joining {
                job_id: job.to_owned(),
                place_id,
            })
        }
        "GameJoinLoadTime" => {
            let r = rest.strip_prefix("Report game_join_loadtime:")?;
            Some(LogEvent::JoinInfo {
                place_id: field(r, "placeid:"),
                universe_id: field(r, "universeid:"),
            })
        }
        "Network" => {
            if rest.starts_with("serverId:") {
                Some(LogEvent::Connected)
            } else if let Some(r) = rest
                .strip_prefix("Disconnection Notification. Reason:")
                .or_else(|| rest.strip_prefix("Sending disconnect with reason:"))
            {
                code(r)
            } else {
                None
            }
        }
        "RbxTransportDummyClient" => code(rest.strip_prefix("Disconnected from server for reason:")?),
        // The doTeleport line carries the join script and its ticket: only its start is read.
        "UgcExperienceController" if rest.contains("doTeleport") => Some(LogEvent::Teleporting),
        "SingleSurfaceApp" if rest.starts_with("initiateTeleport") => Some(LogEvent::Teleporting),
        "SingleSurfaceApp" if rest.starts_with("launchUGCGameInternal") => Some(LogEvent::JoinStart),
        "SingleSurfaceApp" if rest.starts_with("leaveUGCGameInternal") => Some(LogEvent::Leaving),
        "SessionTransitionFSM" if rest.starts_with("Teleported.") => Some(LogEvent::Teleporting),
        "SessionTransitionFSM" if rest.starts_with("Entered app session.") => Some(LogEvent::AtMenu),
        _ => None,
    }
}

/// The first number in `s`, for `Player: 285 (DisconnectClientInitiated)` and ` 285`.
fn code(s: &str) -> Option<LogEvent> {
    let start = s.find(|c: char| c.is_ascii_digit())?;
    let code = leading_number(&s[start..])?;
    u32::try_from(code)
        .ok()
        .map(|code| LogEvent::Disconnected { code })
}

fn leading_number(s: &str) -> Option<u64> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    s[..end].parse().ok()
}

/// `name:123` inside a comma separated report.
fn field(s: &str, name: &str) -> Option<u64> {
    let i = s.find(name)?;
    leading_number(&s[i + name.len()..]).filter(|n| *n > 0)
}

/// A server job id: a UUID.
fn is_job_id(s: &str) -> bool {
    s.len() == 36 && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

/// Most bytes read from one log per poll. A big log is caught up over a few polls.
const READ_CHUNK: usize = 1 << 20;
/// A line longer than this is skipped. None of the lines Bonsai reads come close.
const MAX_LINE: usize = 16 << 10;

/// Follows one log file as the player appends to it.
#[derive(Debug)]
pub struct LogTail {
    pub path: PathBuf,
    offset: u64,
    /// The end of the last read, up to the next newline.
    partial: Vec<u8>,
    /// Inside a line longer than [`MAX_LINE`]; skip to its end.
    skipping: bool,
}

impl LogTail {
    pub fn new(path: PathBuf) -> LogTail {
        LogTail {
            path,
            offset: 0,
            partial: Vec::new(),
            skipping: false,
        }
    }

    /// Events in what was written since the last call.
    pub fn read_new(&mut self) -> std::io::Result<Vec<LogEvent>> {
        let mut f = File::open(&self.path)?;
        let len = f.metadata()?.len();
        if len < self.offset {
            // Rewritten from the start.
            self.offset = 0;
            self.partial.clear();
            self.skipping = false;
        }
        if len == self.offset {
            return Ok(Vec::new());
        }
        f.seek(SeekFrom::Start(self.offset))?;
        let mut buf = Vec::with_capacity((len - self.offset).min(READ_CHUNK as u64) as usize);
        f.take(READ_CHUNK as u64).read_to_end(&mut buf)?;
        self.offset += buf.len() as u64;
        Ok(self.feed(&buf))
    }

    fn feed(&mut self, buf: &[u8]) -> Vec<LogEvent> {
        let mut out = Vec::new();
        let mut rest = buf;
        while let Some(nl) = rest.iter().position(|b| *b == b'\n') {
            let (line, after) = rest.split_at(nl);
            rest = &after[1..];
            if self.skipping {
                self.skipping = false;
                continue;
            }
            let ev = if self.partial.is_empty() {
                parse_bytes(line)
            } else {
                self.partial.extend_from_slice(line);
                let ev = parse_bytes(&self.partial);
                self.partial.clear();
                ev
            };
            out.extend(ev);
        }
        if !self.skipping {
            self.partial.extend_from_slice(rest);
            if self.partial.len() > MAX_LINE {
                self.partial.clear();
                self.skipping = true;
            }
        }
        out
    }
}

fn parse_bytes(line: &[u8]) -> Option<LogEvent> {
    let line = String::from_utf8_lossy(line);
    parse_line(line.trim_end_matches('\r'))
}

/// A folder where a player writes its logs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogDir {
    pub dir: PathBuf,
    /// Only files whose name contains this. The Windows and macOS folders also hold Studio's
    /// and the launcher's logs.
    pub name_has: Option<&'static str>,
}

impl LogDir {
    /// The player logs in this folder, newest last, with their modification time.
    /// Links are skipped (Sober's `latest.log` points at a file that is listed anyway).
    pub fn logs(&self) -> Vec<(PathBuf, std::time::SystemTime)> {
        let Ok(rd) = std::fs::read_dir(&self.dir) else {
            return Vec::new();
        };
        let mut out: Vec<(PathBuf, std::time::SystemTime)> = rd
            .flatten()
            .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
            .filter(|e| self.wants(&e.path()))
            .filter_map(|e| Some((e.path(), e.metadata().ok()?.modified().ok()?)))
            .collect();
        out.sort_by_key(|(_, t)| *t);
        out
    }

    fn wants(&self, p: &Path) -> bool {
        let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
            return false;
        };
        name.ends_with(".log") && self.name_has.is_none_or(|n| name.contains(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A Sober session, scrubbed: the account, addresses and session ids are made up. It joins,
    /// teleports twice (with the disconnects a teleport logs), leaves to the menu and closes.
    const SOBER: &str = include_str!("../../tests/fixtures/sober_session.log");

    fn events(text: &str) -> Vec<LogEvent> {
        text.lines().filter_map(parse_line).collect()
    }

    #[test]
    fn reads_a_sober_session() {
        use LogEvent::*;
        let job = |s: &str| Joining {
            job_id: s.into(),
            place_id: 920587237,
        };
        let mut got = events(SOBER);
        got.dedup();
        assert_eq!(
            got,
            vec![
                AppReady,
                JoinStart,
                job("11111111-2222-4333-8444-555555555555"),
                JoinInfo {
                    place_id: Some(920587237),
                    universe_id: Some(383310974),
                },
                Connected,
                Teleporting,
                Disconnected { code: 285 },
                Teleporting,
                job("66666666-7777-4888-9999-aaaaaaaaaaaa"),
                Connected,
                Leaving,
                Disconnected { code: 285 },
                AtMenu,
                CleanExit,
            ]
        );
    }

    #[test]
    fn already_running() {
        let log = "info: app: lifecycle: fs_init delta=149.7ms\n\
                   FATAL: Crash: An instance of Sober is already running.\n\
                   FATAL: Crash: Aborting...\n";
        assert_eq!(events(log), vec![LogEvent::AlreadyRunning]);
    }

    #[test]
    fn windows_and_mac_lines() {
        // Same engine lines without Sober's prefix.
        let l = "2026-11-20T03:50:34.170Z,1.170000,1f2c,6 [FLog::Network] Disconnection Notification. Reason: 267";
        assert_eq!(parse_line(l), Some(LogEvent::Disconnected { code: 267 }));
        let l = "2026-11-20T03:50:34.170Z,1.170000,1f2c,6 [FLog::Output] ! Joining game 'abcdef01-2345-4678-9abc-def012345678' place 1818 at 10.0.0.1";
        assert_eq!(
            parse_line(l),
            Some(LogEvent::Joining {
                job_id: "abcdef01-2345-4678-9abc-def012345678".into(),
                place_id: 1818
            })
        );
    }

    #[test]
    fn ignores_look_alikes() {
        for l in [
            // Exiting from the menu logs a no-op leave.
            "[FLog::SingleSurfaceApp] leaveUGCGame: ... no-op, not in-game",
            // Chat's connection, not the game's.
            "[DFLog::SignalRCoreError] ID: 1 Disconnected - stop() called",
            "[FLog::SingleSurfaceApp] returnToLuaApp: (stage:UGCGame).",
            "[FLog::Output] ! Joining game 'not-a-job' place 1 at 10.0.0.1",
            "[FLog::Network] Connection lost: connectMode: Disconnect ASAP",
            "info: Roblox: hello",
            "",
        ] {
            assert_eq!(parse_line(l), None, "{l}");
        }
    }

    #[test]
    fn tail_buffers_partial_lines_and_skips_huge_ones() {
        let mut t = LogTail::new(PathBuf::new());
        assert!(t.feed(b"[FLog::Network] server").is_empty());
        assert_eq!(t.feed(b"Id: 1|2\n"), vec![LogEvent::Connected]);
        let huge = vec![b'x'; MAX_LINE + 10];
        assert!(t.feed(&huge).is_empty());
        assert!(
            t.feed(b"[FLog::Network] serverId: still the huge line\n")
                .is_empty()
        );
        assert_eq!(
            t.feed(b"[FLog::SessionTransitionFSM] Entered app session.\r\n"),
            vec![LogEvent::AtMenu]
        );
    }

    #[test]
    fn tail_follows_a_growing_file() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("a.log");
        std::fs::write(&p, "info: app: lifecycle: enter_main_loop delta=0.1ms\n").unwrap();
        let mut t = LogTail::new(p.clone());
        assert_eq!(t.read_new().unwrap(), vec![LogEvent::AppReady]);
        assert!(t.read_new().unwrap().is_empty());
        let mut f = std::fs::OpenOptions::new().append(true).open(&p).unwrap();
        std::io::Write::write_all(&mut f, b"[FLog::Network] serverId: 1|2\n").unwrap();
        assert_eq!(t.read_new().unwrap(), vec![LogEvent::Connected]);
    }

    #[test]
    fn lists_player_logs_only() {
        let dir = tempfile::tempdir().unwrap();
        for n in [
            "0.1_20261120_Player_ab_last.log",
            "0.1_20261120_Studio_ab_last.log",
            "notes.txt",
        ] {
            std::fs::write(dir.path().join(n), "").unwrap();
        }
        let d = LogDir {
            dir: dir.path().into(),
            name_has: Some("Player"),
        };
        let names: Vec<_> = d
            .logs()
            .into_iter()
            .map(|(p, _)| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, ["0.1_20261120_Player_ab_last.log"]);
    }
}
