//! Message text with its links picked out, so chat, private messages and forum posts can open
//! Roblox links in the app (an experience link becomes a card with Join).

use crate::resolve::{Pasted, resolve};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Span {
    Text(String),
    Link {
        url: String,
        /// What a Roblox link points at; `None` for other sites and unknown Roblox pages.
        target: Option<Pasted>,
    },
}

/// Split `text` into plain runs and links (http(s) URLs and bare `roblox.com` links).
pub fn spans(text: &str) -> Vec<Span> {
    let mut out: Vec<Span> = Vec::new();
    let mut plain = String::new();
    let mut rest = text;
    while !rest.is_empty() {
        let at = link_start(rest);
        let Some(i) = at else {
            plain.push_str(rest);
            break;
        };
        plain.push_str(&rest[..i]);
        let tail = &rest[i..];
        let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
        let mut url = &tail[..end];
        // Sentence punctuation after a link is not part of it.
        while let Some(c) = url.chars().last() {
            if ".,!?;:)]}'\"".contains(c) {
                url = &url[..url.len() - c.len_utf8()];
            } else {
                break;
            }
        }
        if url.len() < 8 {
            plain.push_str(&tail[..end.max(1)]);
            rest = &tail[end.max(1)..];
            continue;
        }
        if !plain.is_empty() {
            out.push(Span::Text(std::mem::take(&mut plain)));
        }
        let target = is_roblox(url).then(|| resolve(url)).flatten();
        out.push(Span::Link {
            url: if url.starts_with("http") {
                url.to_string()
            } else {
                format!("https://{url}")
            },
            target,
        });
        rest = &tail[url.len()..];
    }
    if !plain.is_empty() {
        out.push(Span::Text(plain));
    }
    out
}

/// Experiences linked in `text`, in order, without repeats (for Join cards under a message).
pub fn linked_places(text: &str) -> Vec<u64> {
    let mut v = Vec::new();
    for s in spans(text) {
        if let Span::Link {
            target: Some(Pasted::Place(p) | Pasted::Server { place_id: p, .. }),
            ..
        } = s
            && !v.contains(&p)
        {
            v.push(p);
        }
    }
    v
}

fn link_start(s: &str) -> Option<usize> {
    let lower = s.to_ascii_lowercase();
    ["https://", "http://", "www.roblox.com/", "roblox.com/"]
        .iter()
        .filter_map(|p| {
            let mut from = 0;
            while let Some(i) = lower[from..].find(p) {
                let at = from + i;
                // A bare domain must start a word ("myroblox.com/" is not a link).
                let starts_word = at == 0
                    || lower[..at]
                        .chars()
                        .last()
                        .is_some_and(|c| c.is_whitespace() || "([\"'".contains(c));
                if p.starts_with("http") || starts_word {
                    return Some(at);
                }
                from = at + p.len();
            }
            None
        })
        .min()
}

fn is_roblox(url: &str) -> bool {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let host = rest
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    host == "roblox.com" || host.ends_with(".roblox.com")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_out_links() {
        let s = spans(
            "join me https://www.roblox.com/games/1818/Crossroads, now! or see roblox.com/users/156/profile.",
        );
        assert_eq!(s.len(), 5);
        assert_eq!(s[0], Span::Text("join me ".into()));
        assert_eq!(
            s[1],
            Span::Link {
                url: "https://www.roblox.com/games/1818/Crossroads".into(),
                target: Some(Pasted::Place(1818)),
            }
        );
        assert_eq!(s[2], Span::Text(", now! or see ".into()));
        assert!(
            matches!(&s[3], Span::Link { url, target: Some(Pasted::User(156)) } if url == "https://roblox.com/users/156/profile")
        );
        assert_eq!(s[4], Span::Text(".".into()));
    }

    #[test]
    fn other_sites_have_no_target() {
        let s = spans("https://example.com/roblox.com/games/1");
        assert!(matches!(&s[0], Span::Link { target: None, .. }));
        assert_eq!(
            spans("myroblox.com/games/1"),
            vec![Span::Text("myroblox.com/games/1".into())]
        );
        assert_eq!(spans("plain"), vec![Span::Text("plain".into())]);
        assert_eq!(spans(""), vec![]);
    }

    #[test]
    fn places_once_each() {
        assert_eq!(
            linked_places(
                "https://www.roblox.com/games/1818 and https://www.roblox.com/games/1818?x=1 https://www.roblox.com/games/920587237"
            ),
            vec![1818, 920587237]
        );
    }
}
