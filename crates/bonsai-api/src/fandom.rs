//! Other Fandom wikis. Every wiki on Fandom serves the same MediaWiki API as the Roblox Wiki
//! (`specs/undocumented/roblox.fandom.com.yaml`), each on its own subdomain: an experience's
//! own wiki, like `jailbreak.fandom.com`, is the generated operation sent to another host.

use std::collections::HashMap;
use std::sync::OnceLock;

use parking_lot::Mutex;

use crate::generated::roblox_fandom_com::undocumented as fandom;
use crate::{Client, Error, Result};

/// The Roblox Wiki's own name on Fandom, as in `roblox.fandom.com`.
pub const ROBLOX: &str = "roblox";

/// Where every Fandom wiki keeps its images.
pub const IMAGE_HOST: &str = "static.wikia.nocookie.net";

/// Whether `wiki` can be a Fandom subdomain: lowercase letters, digits and dashes.
pub fn valid_wiki(wiki: &str) -> bool {
    (1..=63).contains(&wiki.len())
        && wiki
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !wiki.starts_with('-')
        && !wiki.ends_with('-')
}

/// The host of the wiki called `wiki`, like `jailbreak.fandom.com`. Hosts are `&'static`, so
/// each wiki's is made once and kept; there are as many as wikis the user opened.
pub fn host(wiki: &str) -> Option<&'static str> {
    if wiki == ROBLOX {
        return Some(fandom::HOST);
    }
    if !valid_wiki(wiki) {
        return None;
    }
    static HOSTS: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    let mut hosts = HOSTS.get_or_init(Default::default).lock();
    let base = fandom::HOST.split_once('.').map_or(fandom::HOST, |(_, b)| b);
    Some(
        hosts
            .entry(wiki.to_owned())
            .or_insert_with(|| Box::leak(format!("{wiki}.{base}").into_boxed_str())),
    )
}

/// The wiki a Fandom host belongs to: `jailbreak.fandom.com` → `jailbreak`.
pub fn wiki_of(host: &str) -> Option<&str> {
    let base = fandom::HOST.split_once('.').map_or(fandom::HOST, |(_, b)| b);
    let wiki = host.strip_suffix(base)?.strip_suffix('.')?;
    valid_wiki(wiki).then_some(wiki)
}

/// [`fandom::action_api`] on the wiki called `wiki`.
pub async fn action_api_on(
    client: &Client,
    wiki: &str,
    params: &fandom::ActionApiParams,
) -> Result<fandom::ActionResult> {
    let mut req = fandom::action_api_request(params)?;
    req.host = host(wiki).ok_or_else(|| Error::Encode(format!("not a Fandom wiki: {wiki}")))?;
    client.send(req).await?.json()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hosts() {
        assert_eq!(host("roblox"), Some("roblox.fandom.com"));
        assert_eq!(host("jailbreak"), Some("jailbreak.fandom.com"));
        assert_eq!(host("adopt-me"), Some("adopt-me.fandom.com"));
        assert!(std::ptr::eq(
            host("jailbreak").unwrap(),
            host("jailbreak").unwrap()
        ));
        for bad in ["", "Jailbreak", "a.b", "evil.com/x", "-x", "x-", "a b", "a@b"] {
            assert_eq!(host(bad), None, "{bad}");
        }
        assert_eq!(wiki_of("jailbreak.fandom.com"), Some("jailbreak"));
        assert_eq!(wiki_of("roblox.fandom.com"), Some("roblox"));
        assert_eq!(wiki_of("fandom.com"), None);
        assert_eq!(wiki_of("a.b.fandom.com"), None);
        assert_eq!(wiki_of("jailbreak.fandom.com.evil"), None);
    }
}
