//! Launching the Roblox player: mint an auth ticket for the chosen account, build the
//! `roblox-player:` URI around a PlaceLauncher URL, and hand it to the OS (or a custom player
//! command such as Sober).

use bonsai_api::generated::{assetgame, auth};
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
    use assetgame::undocumented::{PlaceLauncherParams, PlaceLauncherRequest};
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
    assetgame::undocumented::place_launcher_request(&p)
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

/// Ask Roblox for a one-shot ticket for this client's account.
pub async fn mint_ticket(client: &Client) -> Result<Secret, Error> {
    let req = auth::undocumented::post_authentication_ticket_request(
        &auth::undocumented::PostAuthenticationTicketParams {
            referer: "https://www.roblox.com/".into(),
            rbx_authentication_negotiation: Some("1".into()),
        },
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
            None => open::that_detached(uri.expose())
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
                std::process::Command::new(prog)
                    .args(args)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .map(|_| ())
                    .map_err(|e| format!("could not start {prog}: {e}"))
            }
        }
    }
    /// Always the default browser; the player command is only for `roblox-player:`.
    fn open_web(&self, url: &str) -> Result<(), String> {
        open::that_detached(url).map_err(|e| format!("could not open the browser ({e})"))
    }
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
            assert!(s.starts_with("roblox-player:1+launchmode:play+gameinfo:TICKET+launchtime:1700000000000+placelauncherurl:https%3A%2F%2Fassetgame.roblox.com%2Fgame%2FPlaceLauncher.ashx%3F"), "{s}");
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
