//! Finding, and on request closing, the Roblox player on this machine. Only local process
//! lists and folders; nothing here touches the network.
//!
//! - Windows: `RobloxPlayerBeta.exe`, logs in `%LOCALAPPDATA%\Roblox\logs`.
//! - macOS: `Roblox.app/Contents/MacOS/RobloxPlayer`, logs in `~/Library/Logs/Roblox`.
//! - Linux: Sober (a flatpak whose process is `sober`), logs in its `sober_logs` folder; and the
//!   Windows player under Wine, whose process name the kernel cuts to `RobloxPlayerBet`.

use std::path::PathBuf;

use super::log::LogDir;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerKind {
    /// The Roblox player itself (Windows, macOS, or under Wine).
    Roblox,
    Sober,
    /// Roblox's installer or a bootstrapper like Bloxstrap, which starts the player after it
    /// updates. Not the game, but a sign the launch is on its way.
    Bootstrapper,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerProcess {
    pub pid: u32,
    pub kind: PlayerKind,
}

impl PlayerProcess {
    pub fn is_game(&self) -> bool {
        self.kind != PlayerKind::Bootstrapper
    }
}

/// What a process name is, from its executable's file name.
pub fn classify(name: &str) -> Option<PlayerKind> {
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name).trim();
    let lower = base.to_ascii_lowercase();
    let stem = lower.strip_suffix(".exe").unwrap_or(&lower);
    match stem {
        "sober" => Some(PlayerKind::Sober),
        // `RobloxPlayerBet` is Linux's 15 character cut of the Wine process name.
        "robloxplayerbeta" | "robloxplayerbet" | "robloxplayer" => Some(PlayerKind::Roblox),
        "robloxplayerlauncher"
        | "robloxplayerlau"
        | "robloxplayerinstaller"
        | "robloxplayerins"
        | "bloxstrap"
        | "fishstrap"
        | "voidstrap" => Some(PlayerKind::Bootstrapper),
        _ => None,
    }
}

/// `ps -axo pid=,comm=` output (macOS): a pid, then the executable path.
pub fn parse_ps(out: &str) -> Vec<PlayerProcess> {
    out.lines()
        .filter_map(|l| {
            let (pid, comm) = l.trim().split_once(char::is_whitespace)?;
            let pid = pid.parse().ok()?;
            Some(PlayerProcess {
                pid,
                kind: classify(comm.trim())?,
            })
        })
        .collect()
}

/// `tasklist /FO CSV /NH` output (Windows): `"Image Name","PID",…`.
pub fn parse_tasklist(out: &str) -> Vec<PlayerProcess> {
    out.lines()
        .filter_map(|l| {
            let mut cols = l.split("\",\"");
            let name = cols.next()?.trim().trim_start_matches('"');
            let pid = cols.next()?.trim_matches('"').parse().ok()?;
            Some(PlayerProcess {
                pid,
                kind: classify(name)?,
            })
        })
        .collect()
}

/// Running players and bootstrappers.
pub fn find() -> Vec<PlayerProcess> {
    imp::find()
}

/// Asks the player to close (`force` kills it).
pub fn close(pid: u32, force: bool) -> Result<(), String> {
    imp::close(pid, force)
}

/// Where the player on this OS writes its logs.
pub fn log_dirs() -> Vec<LogDir> {
    let Some(b) = directories::BaseDirs::new() else {
        return Vec::new();
    };
    if cfg!(windows) {
        vec![LogDir {
            dir: b.data_local_dir().join("Roblox").join("logs"),
            name_has: Some("Player"),
        }]
    } else if cfg!(target_os = "macos") {
        vec![LogDir {
            dir: b.home_dir().join("Library/Logs/Roblox"),
            name_has: Some("Player"),
        }]
    } else {
        vec![LogDir {
            dir: sober_data(b.home_dir()).join("sober_logs"),
            name_has: None,
        }]
    }
}

/// Sober's data folder inside its flatpak sandbox.
pub fn sober_data(home: &std::path::Path) -> PathBuf {
    home.join(".var/app/org.vinegarhq.Sober/data/sober")
}

#[cfg(target_os = "linux")]
mod imp {
    use super::*;

    pub fn find() -> Vec<PlayerProcess> {
        let Ok(rd) = std::fs::read_dir("/proc") else {
            return Vec::new();
        };
        rd.flatten()
            .filter_map(|e| {
                let pid: u32 = e.file_name().to_str()?.parse().ok()?;
                let comm = std::fs::read_to_string(e.path().join("comm")).ok()?;
                Some(PlayerProcess {
                    pid,
                    kind: classify(comm.trim_end())?,
                })
            })
            .collect()
    }

    pub fn close(pid: u32, force: bool) -> Result<(), String> {
        super::unix_kill(pid, force)
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use super::*;

    pub fn find() -> Vec<PlayerProcess> {
        std::process::Command::new("ps")
            .args(["-axo", "pid=,comm="])
            .stdin(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()
            .map(|o| parse_ps(&String::from_utf8_lossy(&o.stdout)))
            .unwrap_or_default()
    }

    pub fn close(pid: u32, force: bool) -> Result<(), String> {
        super::unix_kill(pid, force)
    }
}

#[cfg(windows)]
mod imp {
    use std::os::windows::process::CommandExt;

    use super::*;

    /// Keeps a console window from flashing up for each call.
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    fn quiet(prog: &str) -> std::process::Command {
        let mut c = std::process::Command::new(prog);
        c.creation_flags(CREATE_NO_WINDOW)
            .stdin(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        c
    }

    pub fn find() -> Vec<PlayerProcess> {
        quiet("tasklist")
            .args(["/FO", "CSV", "/NH"])
            .output()
            .map(|o| parse_tasklist(&String::from_utf8_lossy(&o.stdout)))
            .unwrap_or_default()
    }

    pub fn close(pid: u32, force: bool) -> Result<(), String> {
        let pid = pid.to_string();
        let mut c = quiet("taskkill");
        c.args(["/PID", pid.as_str()]).stdout(std::process::Stdio::null());
        if force {
            c.arg("/F");
        }
        match c.status() {
            Ok(s) if s.success() => Ok(()),
            Ok(s) => Err(format!("taskkill exited with {s}")),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
mod imp {
    use super::*;

    pub fn find() -> Vec<PlayerProcess> {
        Vec::new()
    }

    pub fn close(pid: u32, force: bool) -> Result<(), String> {
        super::unix_kill(pid, force)
    }
}

#[cfg(unix)]
fn unix_kill(pid: u32, force: bool) -> Result<(), String> {
    let status = std::process::Command::new("kill")
        .arg(if force { "-KILL" } else { "-TERM" })
        .arg(pid.to_string())
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("kill exited with {status}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(classify("sober"), Some(PlayerKind::Sober));
        assert_eq!(classify("RobloxPlayerBet"), Some(PlayerKind::Roblox));
        assert_eq!(classify("RobloxPlayerBeta.exe"), Some(PlayerKind::Roblox));
        assert_eq!(
            classify("/Applications/Roblox.app/Contents/MacOS/RobloxPlayer"),
            Some(PlayerKind::Roblox)
        );
        assert_eq!(classify("C:\\x\\Bloxstrap.exe"), Some(PlayerKind::Bootstrapper));
        assert_eq!(classify("RobloxStudioBeta.exe"), None);
        assert_eq!(classify("soberd"), None);
        assert_eq!(classify("RobloxCrashHandler.exe"), None);
    }

    #[test]
    fn mac_ps() {
        let out = "    1 /sbin/launchd\n  812 /Applications/Roblox.app/Contents/MacOS/RobloxPlayer\n  900 /Applications/Roblox.app/Contents/MacOS/RobloxPlayerInstaller.app/Contents/MacOS/RobloxPlayerInstaller\n";
        assert_eq!(
            parse_ps(out),
            vec![
                PlayerProcess {
                    pid: 812,
                    kind: PlayerKind::Roblox
                },
                PlayerProcess {
                    pid: 900,
                    kind: PlayerKind::Bootstrapper
                },
            ]
        );
    }

    #[test]
    fn windows_tasklist() {
        let out = "\"System Idle Process\",\"0\",\"Services\",\"0\",\"8 K\"\r\n\"RobloxPlayerBeta.exe\",\"10432\",\"Console\",\"1\",\"512,004 K\"\r\n\"Bloxstrap.exe\",\"77\",\"Console\",\"1\",\"9 K\"\r\n";
        assert_eq!(
            parse_tasklist(out),
            vec![
                PlayerProcess {
                    pid: 10432,
                    kind: PlayerKind::Roblox
                },
                PlayerProcess {
                    pid: 77,
                    kind: PlayerKind::Bootstrapper
                },
            ]
        );
    }

    #[test]
    fn log_dir_for_this_os() {
        let d = log_dirs();
        assert_eq!(d.len(), 1);
        if cfg!(target_os = "linux") {
            assert!(
                d[0].dir
                    .ends_with(".var/app/org.vinegarhq.Sober/data/sober/sober_logs")
            );
            assert_eq!(d[0].name_has, None);
        }
    }
}
