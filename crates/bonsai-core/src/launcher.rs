//! Launching the Roblox player: mint an auth ticket for the chosen account, build the
//! `roblox-player:` URI around a PlaceLauncher URL, and hand it to the OS (or a custom player
//! command such as Sober).

use bonsai_api::generated::{auth, www};
use bonsai_api::{Client, Error, Secret};

/// The four join modes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinTarget {
    Place {
        place_id: u64,
    },
    Server {
        place_id: u64,
        job_id: String,
    },
    FollowUser {
        user_id: u64,
    },
    PrivateServer {
        place_id: u64,
        link_code: Option<String>,
        access_code: Option<String>,
    },
}

impl JoinTarget {
    pub fn place_id(&self) -> Option<u64> {
        match self {
            JoinTarget::Place { place_id }
            | JoinTarget::Server { place_id, .. }
            | JoinTarget::PrivateServer { place_id, .. } => Some(*place_id),
            JoinTarget::FollowUser { .. } => None,
        }
    }
}

/// `xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx`
pub fn uuid_v4() -> String {
    let mut b = [0u8; 16];
    for x in &mut b {
        *x = fastrand::u8(..);
    }
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h: String = b.iter().map(|x| format!("{x:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &h[0..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..32]
    )
}

/// The PlaceLauncher URL the player will request.
pub fn place_launcher_url(target: &JoinTarget, browser_tracker_id: u64, join_attempt_id: &str) -> String {
    use www::undocumented::{PlaceLauncherParams, PlaceLauncherRequest};
    let mut p = PlaceLauncherParams {
        browser_tracker_id: Some(browser_tracker_id as i64),
        is_play_together_game: Some(false),
        join_attempt_id: Some(join_attempt_id.to_string()),
        join_attempt_origin: Some("PlayButton".into()),
        ..Default::default()
    };
    match target {
        JoinTarget::Place { place_id } => {
            p.request = PlaceLauncherRequest::RequestGame;
            p.place_id = Some(*place_id as i64);
        }
        JoinTarget::Server { place_id, job_id } => {
            p.request = PlaceLauncherRequest::RequestGameJob;
            p.place_id = Some(*place_id as i64);
            p.game_id = Some(job_id.clone());
            p.join_attempt_origin = Some("publicServerListJoin".into());
        }
        JoinTarget::FollowUser { user_id } => {
            p.request = PlaceLauncherRequest::RequestFollowUser;
            p.user_id = Some(*user_id as i64);
            p.join_attempt_origin = Some("JoinUser".into());
        }
        JoinTarget::PrivateServer {
            place_id,
            link_code,
            access_code,
        } => {
            p.request = PlaceLauncherRequest::RequestPrivateGame;
            p.place_id = Some(*place_id as i64);
            p.link_code = link_code.clone();
            p.access_code = access_code.clone();
            p.join_attempt_origin = Some("privateServerListJoin".into());
        }
    }
    // Building a request has no failure mode without a body.
    www::undocumented::place_launcher_request(&p)
        .map(|r| r.url())
        .unwrap_or_default()
}

/// The full launch URI. Contains the ticket, so it is a [`Secret`].
pub fn launch_uri(
    ticket: &Secret,
    target: &JoinTarget,
    browser_tracker_id: u64,
    now_ms: u64,
    locale: &str,
) -> Secret {
    let url = place_launcher_url(target, browser_tracker_id, &uuid_v4());
    Secret::new(format!(
        "roblox-player:1+launchmode:play+gameinfo:{}+launchtime:{now_ms}+placelauncherurl:{}+browsertrackerid:{browser_tracker_id}+robloxLocale:{locale}+gameLocale:{locale}+channel:+LaunchExp:InApp",
        ticket.expose(),
        bonsai_api::rt::encode(&url),
    ))
}

/// Ask Roblox for a one-shot ticket for this client's account. Like the site, first get a
/// client assertion and send it with the ticket request: the player does not join with a
/// ticket minted without one. A failed assertion still tries the ticket, as the site does.
pub async fn mint_ticket(client: &Client) -> Result<Secret, Error> {
    let client_assertion = auth::v1::get_client_assertion(client)
        .await
        .ok()
        .and_then(|a| a.client_assertion)
        .filter(|a| !a.is_empty());
    let req = auth::undocumented::post_authentication_ticket_request(
        &auth::undocumented::PostAuthenticationTicketParams {
            referer: "https://www.roblox.com/".into(),
            rbx_authentication_negotiation: Some("1".into()),
        },
        &auth::undocumented::AuthenticationTicketRequest { client_assertion },
    )?;
    let resp = client.send(req).await?;
    resp.header("rbx-authentication-ticket")
        .filter(|t| !t.is_empty())
        .map(Secret::new)
        .ok_or_else(|| Error::Decode {
            message: "no rbx-authentication-ticket header".into(),
            body: String::new(),
        })
}

/// Hands the URI to the player, and website links to the browser.
pub trait Opener: Send + Sync {
    fn open(&self, uri: &Secret) -> Result<(), String>;
    fn open_web(&self, url: &str) -> Result<(), String>;
}

/// The OS handler for `roblox-player:`, or a custom command with `{uri}` in it
/// (e.g. `flatpak run org.vinegarhq.Sober {uri}`).
#[derive(Default)]
pub struct SystemOpener {
    pub command: Option<String>,
}

impl Opener for SystemOpener {
    fn open(&self, uri: &Secret) -> Result<(), String> {
        match self.command.as_deref().map(str::trim).filter(|c| !c.is_empty()) {
            None => open_detached(uri.expose())
                .map_err(|e| format!("no app is registered for roblox-player: links ({e})")),
            Some(cmd) => {
                let mut parts: Vec<String> = cmd.split_whitespace().map(str::to_string).collect();
                if !parts.iter().any(|p| p.contains("{uri}")) {
                    parts.push("{uri}".into());
                }
                let parts: Vec<String> = parts
                    .into_iter()
                    .map(|p| p.replace("{uri}", uri.expose()))
                    .collect();
                let (prog, args) = parts.split_first().ok_or("empty player command")?;
                let mut cmd = std::process::Command::new(prog);
                cmd.args(args);
                spawn_reaped(&mut cmd).map_err(|e| format!("could not start {prog}: {e}"))
            }
        }
    }
    /// Always the default browser; the player command is only for `roblox-player:`.
    fn open_web(&self, url: &str) -> Result<(), String> {
        open_detached(url).map_err(|e| format!("could not open the browser ({e})"))
    }
}

/// Opens a link or file with the system handler, detached from Bonsai.
///
/// Inside an AppImage the handler gets the environment from before the AppImage runtime
/// changed it (see [`appimage_env_fixes`]); elsewhere this is [`open::that_detached`].
pub fn open_detached(target: &str) -> std::io::Result<()> {
    if std::env::var_os("APPDIR").is_none() {
        return open::that_detached(target);
    }
    let mut last = None;
    for mut cmd in open::commands(target) {
        match spawn_reaped(&mut cmd) {
            Ok(()) => return Ok(()),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| std::io::Error::other("no opener found")))
}

/// Spawns with null stdio and the AppImage environment undone, and reaps the child on a
/// background thread so it never lingers as a zombie.
fn spawn_reaped(cmd: &mut std::process::Command) -> std::io::Result<()> {
    if let Ok(appdir) = std::env::var("APPDIR") {
        for (k, v) in appimage_env_fixes(&appdir, std::env::vars()) {
            match v {
                Some(v) => cmd.env(k, v),
                None => cmd.env_remove(k),
            };
        }
    }
    let mut child = cmd
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    std::thread::spawn(move || child.wait());
    Ok(())
}

/// Variables the AppImage runtime sets for Bonsai itself that must not reach other programs.
const APPIMAGE_ONLY: &[&str] = &["APPDIR", "APPIMAGE", "ARGV0", "OWD", "PYTHONDONTWRITEBYTECODE"];

/// The AppImage runtime points `LD_LIBRARY_PATH`, `PATH`, `GIO_MODULE_DIR` and others into its
/// mount so Bonsai finds its bundled libraries. Programs Bonsai starts inherit that and load the
/// wrong glib: `flatpak`, and so Sober, dies with a symbol lookup error. This drops every list
/// entry under `appdir` or another AppImage mount beside it, removing a variable that is left
/// empty, plus [`APPIMAGE_ONLY`].
/// Returns the changes: `Some` to set, `None` to remove.
pub fn appimage_env_fixes(
    appdir: &str,
    vars: impl IntoIterator<Item = (String, String)>,
) -> Vec<(String, Option<String>)> {
    let appdir = appdir.trim_end_matches('/');
    // An AppImage started from another one (the updater's restart does this) inherits that
    // one's mount too, so strip every `.mount_*` next to `appdir`, not only `appdir`.
    let mounts = appdir
        .rsplit_once('/')
        .map(|(parent, _)| format!("{parent}/.mount_"));
    let inside = |p: &str| {
        !appdir.is_empty()
            && (p == appdir
                || p.starts_with(&format!("{appdir}/"))
                || mounts.as_deref().is_some_and(|m| p.starts_with(m)))
    };
    let mut out = Vec::new();
    for (k, v) in vars {
        if APPIMAGE_ONLY.contains(&k.as_str()) {
            out.push((k, None));
            continue;
        }
        if !v.split(':').any(inside) {
            continue;
        }
        let kept: Vec<&str> = v.split(':').filter(|p| !p.is_empty() && !inside(p)).collect();
        out.push((k, (!kept.is_empty()).then(|| kept.join(":"))));
    }
    out
}

/// Records URIs instead of opening them (tests).
#[derive(Default)]
pub struct RecordingOpener {
    pub opened: parking_lot::Mutex<Vec<Secret>>,
    pub web: parking_lot::Mutex<Vec<String>>,
}

impl Opener for RecordingOpener {
    fn open(&self, uri: &Secret) -> Result<(), String> {
        self.opened.lock().push(uri.clone());
        Ok(())
    }
    fn open_web(&self, url: &str) -> Result<(), String> {
        self.web.lock().push(url.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appimage_env_is_undone() {
        let m = "/tmp/.mount_bonsaiX";
        let vars = [
            ("LD_LIBRARY_PATH", format!("{m}/usr/lib/:{m}/usr/lib32/")),
            ("PATH", format!("{m}/usr/bin/:/usr/local/bin:/usr/bin")),
            ("XDG_DATA_DIRS", format!("{m}/usr/share/:/usr/share")),
            ("GIO_MODULE_DIR", format!("{m}//usr/lib/gio/modules")),
            ("PYTHONPATH", format!("{m}/usr/share/pyshared/:")),
            ("APPIMAGE", "/home/u/bonsai.appimage".into()),
            ("HOME", "/home/u".into()),
            ("OTHER", "/tmp/.mount_bonsaiXY/lib".into()),
            ("KEEP", "/tmp/.mountain/lib:/opt/x".into()),
        ]
        .map(|(k, v)| (k.to_string(), v));
        let mut fixes = appimage_env_fixes(&format!("{m}/"), vars);
        fixes.sort();
        let s = |k: &str, v: Option<&str>| (k.to_string(), v.map(str::to_string));
        assert_eq!(
            fixes,
            vec![
                s("APPIMAGE", None),
                s("GIO_MODULE_DIR", None),
                s("LD_LIBRARY_PATH", None),
                s("OTHER", None),
                s("PATH", Some("/usr/local/bin:/usr/bin")),
                s("PYTHONPATH", None),
                s("XDG_DATA_DIRS", Some("/usr/share")),
            ]
        );
    }

    #[test]
    fn uri_shape_for_each_mode() {
        let t = Secret::new("TICKET");
        let modes = [
            (JoinTarget::Place { place_id: 1818 }, "request%3DRequestGame%26"),
            (
                JoinTarget::Server {
                    place_id: 1818,
                    job_id: "job-1".into(),
                },
                "gameId%3Djob-1",
            ),
            (
                JoinTarget::FollowUser { user_id: 156 },
                "request%3DRequestFollowUser",
            ),
            (
                JoinTarget::PrivateServer {
                    place_id: 1818,
                    link_code: Some("123".into()),
                    access_code: None,
                },
                "linkCode%3D123",
            ),
        ];
        for (m, needle) in modes {
            let uri = launch_uri(&t, &m, 4242, 1_700_000_000_000, "en_us");
            let s = uri.expose();
            assert!(s.starts_with("roblox-player:1+launchmode:play+gameinfo:TICKET+launchtime:1700000000000+placelauncherurl:https%3A%2F%2Fwww.roblox.com%2FGame%2FPlaceLauncher.ashx%3F"), "{s}");
            assert!(s.contains(needle), "{needle} not in {s}");
            assert!(s.ends_with(
                "+browsertrackerid:4242+robloxLocale:en_us+gameLocale:en_us+channel:+LaunchExp:InApp"
            ));
            assert_eq!(format!("{uri:?}"), "<redacted>");
        }
    }

    #[test]
    fn uuid_format() {
        let u = uuid_v4();
        assert_eq!(u.len(), 36);
        assert_eq!(&u[14..15], "4");
    }
}
