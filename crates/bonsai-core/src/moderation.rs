//! Account moderation: warnings, bans, terminations and voice chat suspensions.
//!
//! Roblox answers most requests of a moderated account with 403 "User is moderated",
//! `users/authenticated` included, while the cookie itself stays valid. [`is_moderated`] spots
//! that answer. The account's session becomes [`SessionState::Moderated`] and
//! `usermoderation/v1/not-approved` says what happened. Only a later successful
//! `users/authenticated` ends it: a reactivation here, or one on the website.
//!
//! Voice chat suspensions are separate: the account works, and `voice/v1/settings` says
//! `isBanned` with an end time.

use bonsai_api::generated::{usermoderation, users, voice};
use bonsai_api::{AccountId, Client, Error};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::cache::Kind;
use crate::core::Inner;
use crate::store::{SessionState, StoreData, UserInfo};

/// The message of Roblox's 403 for a moderated account.
const MODERATED: &str = "User is moderated";
/// Longest Roblox explanation kept.
const MAX_TEXT: usize = 2000;
/// Roblox keeps answering with the moderation for a moment after a reactivation; the website
/// waits this long before reloading.
const REACTIVATE_SETTLE: std::time::Duration = std::time::Duration::from_secs(2);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModerationKind {
    /// Must be accepted on Roblox (or here) before the account works again.
    Warned,
    /// Temporary when `ends` is set, otherwise until Roblox lifts it.
    Banned,
    /// Deleted. Never reactivates.
    Terminated,
    /// No punishment, but Roblox wants a human check on the website first.
    Locked,
    #[default]
    Unknown,
}

/// What Roblox did to an account and why, as `not-approved` reports it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Moderation {
    pub kind: ModerationKind,
    /// Roblox's heading for the notice, when it wrote one.
    pub title: String,
    /// Roblox's explanation to the user.
    pub message: String,
    /// Unix seconds; 0 when unknown.
    pub begins: i64,
    /// Unix seconds when a ban ends; None for everything else and for bans without an end.
    pub ends: Option<i64>,
}

impl Moderation {
    /// Roblox accepts a reactivation for a warning, or for a ban that has ended. Never for a
    /// termination or a lock.
    pub fn can_reactivate(&self, now: i64) -> bool {
        match self.kind {
            ModerationKind::Warned => true,
            ModerationKind::Banned => self.ends.is_some_and(|e| now >= e),
            _ => false,
        }
    }
}

/// A voice chat suspension.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceBan {
    /// Unix seconds; None when Roblox didn't say.
    pub until: Option<i64>,
}

/// Roblox's answer for an account it moderated. Other 403s (a challenge, a missing
/// permission) are not.
pub fn is_moderated(e: &Error) -> bool {
    matches!(e, Error::Status { status: 403, errors, .. }
        if errors.first().is_some_and(|x| x.message == MODERATED))
}

/// Reads `not-approved`. The punishment type is "Warn", "Delete" for a termination, "Ban" and
/// a length ("Ban 7 Days"), or empty for an account locked until a human check.
pub fn from_not_approved(r: usermoderation::undocumented::NotApproved) -> Moderation {
    let kind = match r.punishment_type_description.as_deref().map(str::trim) {
        None | Some("") => ModerationKind::Locked,
        Some("Warn") => ModerationKind::Warned,
        Some("Delete") => ModerationKind::Terminated,
        Some(t) if t.starts_with("Ban") => ModerationKind::Banned,
        Some(_) => ModerationKind::Unknown,
    };
    let body = text(r.body_);
    let ends = r.end_date.as_deref().and_then(crate::fmt::parse_iso);
    Moderation {
        kind,
        title: text(r.title),
        message: if body.is_empty() {
            text(r.message_to_user)
        } else {
            body
        },
        begins: r
            .begin_date
            .as_deref()
            .and_then(crate::fmt::parse_iso)
            .unwrap_or(0),
        ends: ends.filter(|_| kind == ModerationKind::Banned),
    }
}

fn text(s: Option<String>) -> String {
    s.unwrap_or_default().trim().chars().take(MAX_TEXT).collect()
}

/// The voice suspension in `voice/v1/settings`, if there is one.
pub fn voice_ban(v: &voice::undocumented::VoiceSettings) -> Option<VoiceBan> {
    v.is_banned.unwrap_or(false).then(|| VoiceBan {
        until: v.banned_until.as_ref().and_then(|t| t.seconds).filter(|s| *s > 0),
    })
}

/// Where the details are cached, under [`Kind::Local`], so a restart shows them at once.
pub(crate) const CACHE_ID: &str = "moderation";
/// How often each account's voice settings are read for a suspension.
const VOICE_EVERY: i64 = 600;

/// Marks the account moderated and reads what Roblox did. When `not-approved` fails the
/// account stays moderated with the details it had. Returns the details to cache.
async fn load(store: &RwLock<StoreData>, id: AccountId, c: &Client) -> Option<Moderation> {
    let details = match usermoderation::undocumented::get_not_approved(c).await {
        Ok(r) => Some(from_not_approved(r)),
        Err(e) => {
            tracing::debug!("not-approved for a moderated account: {e}");
            None
        }
    };
    let mut s = store.write();
    let d = s.data.entry(id).or_default();
    d.session = SessionState::Moderated;
    d.voice_ban = None;
    if details.is_some() {
        d.moderation = details.clone();
    }
    details
}

impl Inner {
    /// `refresh_account` got "User is moderated".
    pub(crate) async fn moderated(&self, id: AccountId, c: &Client) {
        if let Some(m) = load(&self.store, id, c).await {
            self.cache.put(Kind::Local, id, CACHE_ID, &m);
        }
        self.repaint();
    }

    /// Some other request got "User is moderated". The next refresh caches the details.
    pub(crate) fn note_moderated(&self, id: AccountId) {
        let known = self
            .store
            .read()
            .account_data(id)
            .is_some_and(|d| d.session == SessionState::Moderated);
        let Some(c) = self.client(id).filter(|_| !known) else {
            return;
        };
        let store = self.store.clone();
        let repaint = self.repaint.clone();
        self.rt.spawn(async move {
            load(&store, id, &c).await;
            repaint();
        });
    }

    /// `users/authenticated` answered, so whatever Roblox did is over.
    pub(crate) fn clear_moderation(&self, id: AccountId) {
        let had = self
            .store
            .write()
            .data
            .get_mut(&id)
            .and_then(|d| d.moderation.take())
            .is_some();
        if had {
            self.cache.delete(Kind::Local, id, CACHE_ID);
        }
    }

    /// Reads the account's voice suspension now and then, for the account rail.
    pub(crate) async fn check_voice(&self, id: AccountId, c: &Client) {
        if !self.due(&format!("voice_ban:{}", id.0), VOICE_EVERY) {
            return;
        }
        match voice::undocumented::get_voice_settings(c).await {
            Ok(v) => {
                let ban = voice_ban(&v);
                let mut s = self.store.write();
                let d = s.data.entry(id).or_default();
                if d.voice_ban != ban {
                    d.voice_ban = ban;
                    drop(s);
                    self.repaint();
                }
            }
            Err(e) => self.note_error(id, &e),
        }
    }
}

/// Who a moderated cookie belongs to, since `users/authenticated` refuses it. The names come
/// from the public profile.
pub(crate) async fn identify(c: &Client, anon: &Client) -> Result<(UserInfo, Moderation), Error> {
    let r = usermoderation::undocumented::get_not_approved(c).await?;
    let id = r.punished_user_id.filter(|id| *id > 0).ok_or_else(|| {
        Error::Message("Roblox moderated this account but didn't say which one it is. Try again.".into())
    })?;
    let moderation = from_not_approved(r);
    let u = users::v1::get_users_by_user_id(anon, id).await?;
    let user = UserInfo {
        id: id as u64,
        name: u.name.unwrap_or_default(),
        display_name: u.display_name.unwrap_or_default(),
        verified: false,
        premium: None,
    };
    Ok((user, moderation))
}

impl Core {
    /// Accept a warning or an ended ban, then check the account again.
    pub fn reactivate(&self, account: AccountId) -> Result<(), String> {
        let now = crate::cache::now();
        let ok = self
            .store()
            .account_data(account)
            .and_then(|d| d.moderation.as_ref())
            .is_some_and(|m| m.can_reactivate(now));
        if !ok {
            return Err("Only a warning or a ban that has ended can be reactivated.".into());
        }
        self.inner.mutate(
            account,
            "reactivate the account",
            "Account reactivated.",
            &[],
            move |f| async move {
                usermoderation::undocumented::post_reactivate(&f.client).await?;
                tokio::time::sleep(REACTIVATE_SETTLE).await;
                f.inner.refresh_account(account).await;
                let still = f
                    .inner
                    .store
                    .read()
                    .account_data(account)
                    .is_some_and(|d| d.session == SessionState::Moderated);
                if still {
                    return Err(Error::Message(
                        "Roblox still reports this account as moderated. Try again in a minute.".into(),
                    ));
                }
                Ok(())
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bonsai_api::ApiError;
    use usermoderation::undocumented::NotApproved;

    fn status(code: u16, message: &str) -> Error {
        Error::Status {
            status: code,
            errors: vec![ApiError {
                message: message.into(),
                ..Default::default()
            }],
            body: String::new(),
        }
    }

    #[test]
    fn spots_the_moderated_403_only() {
        assert!(is_moderated(&status(403, "User is moderated")));
        assert!(!is_moderated(&status(403, "Token Validation Failed")));
        assert!(!is_moderated(&status(400, "User is moderated")));
        assert!(!is_moderated(&Error::Unauthorized));
    }

    fn punished(kind: &str) -> NotApproved {
        NotApproved {
            punished_user_id: Some(1),
            punishment_type_description: Some(kind.into()),
            begin_date: Some("2026-11-02T10:00:00.123Z".into()),
            end_date: Some("2026-11-09T10:00:00Z".into()),
            message_to_user: Some(" Spam ".into()),
            ..Default::default()
        }
    }

    #[test]
    fn reads_each_punishment() {
        let ban = from_not_approved(punished("Ban 7 Days"));
        assert_eq!(ban.kind, ModerationKind::Banned);
        assert_eq!(
            ban.ends.zip(Some(ban.begins)).map(|(e, b)| e - b),
            Some(7 * 86_400)
        );
        assert_eq!(ban.message, "Spam");
        let warn = from_not_approved(punished("Warn"));
        assert_eq!((warn.kind, warn.ends), (ModerationKind::Warned, None));
        assert_eq!(
            from_not_approved(punished("Delete")).kind,
            ModerationKind::Terminated
        );
        assert_eq!(from_not_approved(punished("")).kind, ModerationKind::Locked);
        assert_eq!(from_not_approved(punished("Mute")).kind, ModerationKind::Unknown);
        let mut b = punished("Warn");
        b.body_ = Some("Body wins".into());
        assert_eq!(from_not_approved(b).message, "Body wins");
    }

    #[test]
    fn only_warnings_and_ended_bans_reactivate() {
        let ban = from_not_approved(punished("Ban 7 Days"));
        let end = ban.ends.unwrap();
        assert!(!ban.can_reactivate(end - 1));
        assert!(ban.can_reactivate(end));
        assert!(from_not_approved(punished("Warn")).can_reactivate(0));
        assert!(!from_not_approved(punished("Delete")).can_reactivate(i64::MAX));
        assert!(!from_not_approved(punished("")).can_reactivate(i64::MAX));
        let mut forever = ban;
        forever.ends = None;
        assert!(!forever.can_reactivate(i64::MAX));
    }

    #[test]
    fn reads_voice_bans() {
        let mut v = voice::undocumented::VoiceSettings {
            is_banned: Some(false),
            ..Default::default()
        };
        assert_eq!(voice_ban(&v), None);
        v.is_banned = Some(true);
        assert_eq!(voice_ban(&v), Some(VoiceBan { until: None }));
        v.banned_until = Some(voice::undocumented::Timestamp {
            seconds: Some(1_800_000_000),
        });
        assert_eq!(
            voice_ban(&v),
            Some(VoiceBan {
                until: Some(1_800_000_000)
            })
        );
    }
}
