//! "Paste anything": turn roblox.com URLs, deep links and bare ids into targets.

use url::Url;

/// Something a pasted string points at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pasted {
    Place(u64),
    /// A specific server of a place.
    Server {
        place_id: u64,
        job_id: String,
    },
    /// `privateServerLinkCode` on a game URL.
    PrivateServerLink {
        place_id: u64,
        link_code: String,
    },
    /// `roblox.com/share?code=..&type=Server` (resolved through the share-links API).
    ShareLink {
        code: String,
        kind: String,
    },
    Universe(u64),
    User(u64),
    Asset(u64),
    Bundle(u64),
    Group(u64),
    Badge(u64),
    GamePass(u64),
    /// A bare number: could be a place, universe, user or asset. The caller checks each.
    AnyId(u64),
}

fn num(s: &str) -> Option<u64> {
    s.parse().ok()
}

fn is_roblox_host(h: &str) -> bool {
    h == "roblox.com" || h.ends_with(".roblox.com") || h == "ro.blox.com"
}

/// Returns `None` when the text is not something we can resolve (treat it as a search query).
pub fn resolve(input: &str) -> Option<Pasted> {
    let s = input.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(n) = num(s) {
        return Some(Pasted::AnyId(n));
    }
    if let Some(rest) = s.strip_prefix("roblox://").or_else(|| s.strip_prefix("roblox:")) {
        return deep_link(rest);
    }
    let with_scheme = if s.starts_with("http://") || s.starts_with("https://") {
        s.to_string()
    } else if s.starts_with("www.roblox.com")
        || s.starts_with("roblox.com")
        || s.starts_with("web.roblox.com")
    {
        format!("https://{s}")
    } else {
        return None;
    };
    let u = Url::parse(&with_scheme).ok()?;
    if !u.host_str().is_some_and(is_roblox_host) {
        return None;
    }
    let q = |k: &str| {
        u.query_pairs()
            .find(|(a, _)| a.eq_ignore_ascii_case(k))
            .map(|(_, v)| v.into_owned())
    };
    let segs: Vec<String> = u
        .path_segments()
        .map(|p| p.filter(|s| !s.is_empty()).map(str::to_lowercase).collect())
        .unwrap_or_default();
    // Locale prefixes like /de/games/123
    let segs: Vec<&str> = {
        let v: Vec<&str> = segs.iter().map(String::as_str).collect();
        if v.first()
            .is_some_and(|f| f.len() == 2 && v.len() > 1 && !f.chars().any(|c| c.is_ascii_digit()))
        {
            v[1..].to_vec()
        } else {
            v
        }
    };
    match segs.as_slice() {
        ["share", ..] => Some(Pasted::ShareLink {
            code: q("code")?,
            kind: q("type").unwrap_or_else(|| "Server".into()),
        }),
        ["games", "start", ..] => {
            let place_id = num(&q("placeId")?)?;
            match q("gameInstanceId").or_else(|| q("gameId")) {
                Some(job_id) if !job_id.is_empty() => Some(Pasted::Server { place_id, job_id }),
                _ => Some(Pasted::Place(place_id)),
            }
        }
        ["games", id, ..] => {
            let place_id = num(id)?;
            if let Some(code) = q("privateServerLinkCode") {
                return Some(Pasted::PrivateServerLink {
                    place_id,
                    link_code: code,
                });
            }
            if let Some(job) = q("gameInstanceId") {
                return Some(Pasted::Server {
                    place_id,
                    job_id: job,
                });
            }
            Some(Pasted::Place(place_id))
        }
        ["users", id, ..] => num(id).map(Pasted::User),
        ["catalog", id, ..] | ["library", id, ..] => num(id).map(Pasted::Asset),
        ["bundles", id, ..] => num(id).map(Pasted::Bundle),
        ["communities", id, ..] | ["groups", id, ..] => num(id).map(Pasted::Group),
        ["badges", id, ..] => num(id).map(Pasted::Badge),
        ["game-pass", id, ..] => num(id).map(Pasted::GamePass),
        _ => q("placeId").and_then(|p| num(&p)).map(Pasted::Place),
    }
}

/// `roblox://placeId=1&gameInstanceId=x`, `roblox://experiences/start?placeId=1`,
/// `roblox://navigation/profile?userId=1`.
fn deep_link(rest: &str) -> Option<Pasted> {
    let rest = rest.trim_start_matches('/');
    let (path, query) = match rest.find('?') {
        Some(i) => (&rest[..i], &rest[i + 1..]),
        None if rest.contains('=') => ("", rest),
        None => (rest, ""),
    };
    let pairs: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();
    let q = |k: &str| {
        pairs
            .iter()
            .find(|(a, _)| a.eq_ignore_ascii_case(k))
            .map(|(_, v)| v.clone())
    };
    let path = path.to_lowercase();
    if path.contains("profile") {
        return q("userId").and_then(|v| num(&v)).map(Pasted::User);
    }
    let place_id = q("placeId").and_then(|v| num(&v))?;
    if let Some(code) = q("linkCode").or_else(|| q("privateServerLinkCode")) {
        return Some(Pasted::PrivateServerLink {
            place_id,
            link_code: code,
        });
    }
    match q("gameInstanceId").or_else(|| q("gameId")) {
        Some(job_id) if !job_id.is_empty() => Some(Pasted::Server { place_id, job_id }),
        _ => Some(Pasted::Place(place_id)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Pasted::*;

    #[test]
    fn urls_and_links() {
        let cases: Vec<(&str, Pasted)> = vec![
            (
                "https://www.roblox.com/games/920587237/Adopt-Me",
                Place(920587237),
            ),
            ("roblox.com/games/1818", Place(1818)),
            ("https://www.roblox.com/de/games/1818/Crossroads", Place(1818)),
            (
                "https://www.roblox.com/games/start?placeId=1818&gameInstanceId=abc-123",
                Server {
                    place_id: 1818,
                    job_id: "abc-123".into(),
                },
            ),
            (
                "https://www.roblox.com/games/1818/x?privateServerLinkCode=987",
                PrivateServerLink {
                    place_id: 1818,
                    link_code: "987".into(),
                },
            ),
            (
                "https://www.roblox.com/share?code=abcdef&type=Server",
                ShareLink {
                    code: "abcdef".into(),
                    kind: "Server".into(),
                },
            ),
            ("https://www.roblox.com/users/156/profile", User(156)),
            (
                "https://www.roblox.com/catalog/1365767/Valkyrie-Helm",
                Asset(1365767),
            ),
            ("https://www.roblox.com/bundles/192/Rthro", Bundle(192)),
            ("https://www.roblox.com/communities/7/x", Group(7)),
            ("https://www.roblox.com/groups/7/x", Group(7)),
            ("https://www.roblox.com/badges/2124/x", Badge(2124)),
            ("https://www.roblox.com/game-pass/1/x", GamePass(1)),
            (
                "roblox://placeId=1818&gameInstanceId=job",
                Server {
                    place_id: 1818,
                    job_id: "job".into(),
                },
            ),
            ("roblox://experiences/start?placeId=1818", Place(1818)),
            ("roblox://navigation/profile?userId=156", User(156)),
            ("  920587237 ", AnyId(920587237)),
        ];
        for (input, want) in cases {
            assert_eq!(resolve(input), Some(want), "{input}");
        }
        assert_eq!(resolve("adopt me"), None);
        assert_eq!(resolve("https://example.com/games/1"), None);
        assert_eq!(resolve("https://evilroblox.com/games/1"), None);
    }
}
