//! Website links for "open on website", "copy link" and server invites. Built from the
//! www page specs, never from hardcoded URLs.

use bonsai_api::generated::www;

use crate::settings::TabRef;

fn url(r: bonsai_api::Result<bonsai_api::rt::Request>) -> String {
    r.map(|r| r.url()).unwrap_or_default()
}

pub fn tab(t: &TabRef) -> String {
    match t {
        TabRef::Experience { place_id } => url(www::undocumented::page_game_request(*place_id as i64)),
        TabRef::Profile { user_id } => url(www::undocumented::page_profile_request(*user_id as i64)),
        TabRef::Item { asset_id } => url(www::undocumented::page_item_request(*asset_id as i64)),
        TabRef::Bundle { bundle_id } => url(www::undocumented::page_bundle_request(*bundle_id as i64)),
        TabRef::Community { group_id } => url(www::undocumented::page_community_request(*group_id as i64)),
    }
}

/// Section pages by key.
pub fn section(key: &str, user_id: Option<u64>) -> Option<String> {
    Some(match key {
        "home" => url(www::undocumented::page_home_request()),
        "discover" => url(www::undocumented::page_charts_request()),
        "avatar" => url(www::undocumented::page_avatar_request()),
        "marketplace" => url(www::undocumented::page_marketplace_request()),
        "inventory" => url(www::undocumented::page_inventory_request(user_id? as i64)),
        "trades" => url(www::undocumented::page_trades_request()),
        "messages" => url(www::undocumented::page_messages_request()),
        "communities" => url(www::undocumented::page_communities_request()),
        "settings" => url(www::undocumented::page_settings_request()),
        "friends" => url(www::undocumented::page_friends_request()),
        // Robux are only ever bought on the website.
        "robux" => url(www::undocumented::page_buy_robux_request()),
        _ => return None,
    })
}

/// The site's report form for a user, place, asset or group. Opened in the browser only.
pub fn report(vector: &str, target_id: u64, submitter: Option<u64>) -> String {
    url(www::undocumented::page_report_abuse_request(
        &www::undocumented::PageReportAbuseParams {
            target_id: target_id as i64,
            submitter_id: submitter.map(|s| s as i64),
            abuse_vector: vector.to_string(),
        },
    ))
}

/// A pass's page on the website, where it can be bought.
pub fn game_pass(pass_id: u64) -> String {
    url(www::undocumented::page_game_pass_request(pass_id as i64))
}

/// A link that opens a specific server for whoever clicks it.
pub fn server_invite(place_id: u64, job_id: &str) -> String {
    url(www::undocumented::page_games_start_request(
        &www::undocumented::PageGamesStartParams {
            place_id: place_id as i64,
            game_instance_id: Some(job_id.to_string()),
        },
    ))
}

/// The page for a target like `game:1818`, `user:1`, `item:5`, `bundle:5`, `group:7`,
/// `pass:9` or `section:trades`: the forms the UI passes as a command argument or page
/// context. `me` fills in sections that need the signed-in user.
pub fn page(target: &str, me: Option<u64>) -> Option<String> {
    let (kind, rest) = split(target)?;
    if kind == "section" {
        return section(rest, me);
    }
    let id: u64 = rest.parse().ok()?;
    Some(match kind {
        "game" => tab(&TabRef::Experience { place_id: id }),
        "user" => tab(&TabRef::Profile { user_id: id }),
        "item" => tab(&TabRef::Item { asset_id: id }),
        "bundle" => tab(&TabRef::Bundle { bundle_id: id }),
        "group" => tab(&TabRef::Community { group_id: id }),
        "pass" => game_pass(id),
        _ => return None,
    })
}

/// The id in a target like `user:1` or `outfit:5`, for "Copy ID".
pub fn target_id(target: &str) -> Option<u64> {
    split(target)?.1.parse().ok()
}

/// `kind:id` of a target. Anything after a `/` (like `group:7/forum/...`, a page's open
/// post) is not part of the id.
fn split(target: &str) -> Option<(&str, &str)> {
    let (kind, rest) = target.trim().split_once(':')?;
    Some((kind, rest.split('/').next()?.trim()))
}

/// The report form for a target in the same forms as [`page`]. Bundles, passes and sections
/// have none.
pub fn report_target(target: &str, me: Option<u64>) -> Option<String> {
    let (kind, rest) = split(target)?;
    let id: u64 = rest.parse().ok()?;
    let vector = match kind {
        "game" => "place",
        "user" => "user",
        "item" => "asset",
        "group" => "group",
        _ => return None,
    };
    Some(report(vector, id, me))
}

/// Whether a web link points at a Roblox site (www/web/help). Notification links are only
/// opened when this holds.
pub fn is_roblox_url(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else {
        return false;
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    host == "roblox.com" || host.ends_with(".roblox.com")
}

/// The experience a "Share experience" argument names: a place id, or a link to an experience
/// or one of its servers. Private server links are left out so a share never hands one over.
pub fn shared_place(arg: &str) -> Option<u64> {
    use crate::resolve::{Pasted, resolve};
    match resolve(arg)? {
        Pasted::Place(id) | Pasted::AnyId(id) | Pasted::Server { place_id: id, .. } if id > 0 => Some(id),
        _ => None,
    }
}

/// The Roblox sign-in page, loaded in the app's sign-in window.
pub fn sign_in_page() -> String {
    url(www::undocumented::page_login_request())
}

/// Arkose's puzzle script for one public key, loaded by the captcha window.
pub fn captcha_script(public_key: &str) -> String {
    url(bonsai_api::generated::arkoselabs::undocumented::captcha_script_request(public_key))
}

/// Hosts the sign-in window may load as a page or frame: Roblox itself and the captcha
/// provider its sign-in page embeds. Everything else is blocked.
pub fn sign_in_window_allows(host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    let under = |domain: &str| host == domain || host.ends_with(&format!(".{domain}"));
    under("roblox.com") || under("arkoselabs.com")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_in_window_hosts() {
        assert_eq!(sign_in_page(), "https://www.roblox.com/login");
        assert!(sign_in_window_allows("www.roblox.com"));
        assert!(sign_in_window_allows("roblox.com"));
        assert!(sign_in_window_allows("roblox-api.arkoselabs.com"));
        assert!(!sign_in_window_allows("roblox.com.example.com"));
        assert!(!sign_in_window_allows("notroblox.com"));
        assert!(!sign_in_window_allows("example.com"));
        assert!(sign_in_window_allows("arkoselabs.roblox.com"));
        assert_eq!(
            captcha_script("ABC-1"),
            "https://arkoselabs.roblox.com/v2/ABC-1/api.js"
        );
    }

    #[test]
    fn links() {
        assert_eq!(
            tab(&TabRef::Experience { place_id: 1818 }),
            "https://www.roblox.com/games/1818"
        );
        assert_eq!(
            tab(&TabRef::Profile { user_id: 1 }),
            "https://www.roblox.com/users/1/profile"
        );
        assert_eq!(
            server_invite(1818, "abc"),
            "https://www.roblox.com/games/start?placeId=1818&gameInstanceId=abc"
        );
        assert_eq!(
            section("trades", None).as_deref(),
            Some("https://www.roblox.com/trades")
        );
        assert!(section("inventory", None).is_none());
        assert_eq!(
            report("user", 156, Some(2)),
            "https://www.roblox.com/report-abuse/?targetId=156&submitterId=2&abuseVector=user"
        );
    }

    #[test]
    fn shared_places() {
        assert_eq!(shared_place("1818"), Some(1818));
        assert_eq!(
            shared_place(" https://www.roblox.com/games/1818/Crossroads "),
            Some(1818)
        );
        assert_eq!(
            shared_place("https://www.roblox.com/games/start?placeId=1818&gameInstanceId=abc"),
            Some(1818)
        );
        assert_eq!(shared_place("https://www.roblox.com/users/1/profile"), None);
        assert_eq!(shared_place("https://example.com/games/1818"), None);
        assert_eq!(shared_place("0"), None);
        assert_eq!(shared_place("hello"), None);
    }

    #[test]
    fn targets() {
        assert_eq!(
            page("game:1818", None).as_deref(),
            Some("https://www.roblox.com/games/1818")
        );
        assert_eq!(
            page(" user:1 ", None).as_deref(),
            Some("https://www.roblox.com/users/1/profile")
        );
        assert_eq!(page("pass:9", None), Some(game_pass(9)));
        assert!(page("section:robux", None).is_some_and(|u| u.ends_with("/upgrades/robux")));
        assert_eq!(page("section:inventory", Some(4)), section("inventory", Some(4)));
        assert_eq!(page("section:inventory", None), None);
        assert_eq!(page("game:abc", None), None);
        assert_eq!(page("1818", None), None);
        assert_eq!(page("place:1818", None), None);
        assert_eq!(
            report_target("game:1818", Some(2)).as_deref(),
            Some("https://www.roblox.com/report-abuse/?targetId=1818&submitterId=2&abuseVector=place")
        );
        assert_eq!(report_target("pass:9", None), None);
        assert_eq!(target_id("outfit:5"), Some(5));
        assert_eq!(target_id(" user:12 "), Some(12));
        assert_eq!(target_id("section:trades"), None);
        assert_eq!(target_id("12"), None);
        assert_eq!(target_id("group:7/forum/a/b"), Some(7));
        assert_eq!(page("group:7/forum/a/b", None), page("group:7", None));
    }
}
