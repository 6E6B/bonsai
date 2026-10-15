//! Private messages (inbox, sent, archive, news) and the site's notification stream.
//!
//! Roblox no longer accepts new private messages (`/v1/messages/send` answers 404, BLOCKERS
//! #7), so Reply and Compose start a chat instead (see [`crate::chat`]).

use rbx_api::AccountId;
use rbx_api::generated::notifications;
use rbx_api::generated::privatemessages;
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::res::{Key, Page, Res};

const AGE: i64 = 120;
const PAGE: i32 = 20;

/// The message tabs on www.roblox.com/my/messages.
pub const TABS: [(&str, &str); 4] = [
    ("inbox", "Inbox"),
    ("sent", "Sent"),
    ("news", "News"),
    ("archive", "Archive"),
];

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrivateMessage {
    pub id: i64,
    pub sender_id: u64,
    pub sender: String,
    pub sender_verified: bool,
    pub recipient_id: u64,
    pub recipient: String,
    pub subject: String,
    /// Plain text (the site sends HTML).
    pub body: String,
    /// ISO date-time.
    pub created: String,
    pub read: bool,
    pub system: bool,
}

/// Message ids from a command argument: `5` or `5,7,9` (what bulk actions pass). Anything
/// that is not a number is skipped, and repeats count once.
pub fn parse_ids(arg: &str) -> Vec<i64> {
    let mut ids = Vec::new();
    for id in arg.split(',').filter_map(|s| s.trim().parse::<i64>().ok()) {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    ids
}

/// What "toggle read" does to these messages: `true` marks them all read, `false` marks them
/// all unread. Read unless everything already is, like a mail client's bulk toggle.
pub fn mark_read_target(messages: &[PrivateMessage]) -> bool {
    messages.iter().any(|m| !m.read)
}

/// One entry of the notification stream (the bell on the site).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamNotification {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub body: String,
    /// ISO date-time.
    pub when: String,
    pub read: bool,
    /// A web link the notification points to, if it has one.
    pub link: Option<String>,
    /// User whose avatar the site shows next to it.
    pub user_id: Option<u64>,
}

impl Core {
    /// One message tab, 20 at a time. `tab` is a key from [`TABS`].
    pub fn messages(&self, account: AccountId, tab: &str, more: bool) -> Res<Page<PrivateMessage>> {
        if account.is_anonymous() {
            return Res::default();
        }
        let tab = tab.to_string();
        self.inner.paged(
            Key::private(account, "private_messages", &tab),
            AGE,
            more,
            move |f, cursor| {
                let tab = tab.clone();
                async move {
                    if tab == "news" {
                        let r = privatemessages::v1::get_announcements(&f.client).await?;
                        let items = r
                            .collection
                            .unwrap_or_default()
                            .into_iter()
                            .map(|a| {
                                let (sender_id, sender, sender_verified) = user(a.sender);
                                PrivateMessage {
                                    id: a.id.unwrap_or(0),
                                    sender_id,
                                    sender,
                                    sender_verified,
                                    subject: a.subject.unwrap_or_default(),
                                    body: html_to_text(&a.body_.unwrap_or_default()),
                                    created: a.created.unwrap_or_default(),
                                    read: true,
                                    system: true,
                                    ..Default::default()
                                }
                            })
                            .collect();
                        return Ok(Page { items, next: None });
                    }
                    let page: i32 = cursor.and_then(|c| c.parse().ok()).unwrap_or(0);
                    let p = privatemessages::v1::GetMessagesParams {
                        page_number: Some(page),
                        page_size: Some(PAGE),
                        message_tab: Some(match tab.as_str() {
                            "sent" => privatemessages::v1::GetMessagesMessageTab::Sent,
                            "archive" => privatemessages::v1::GetMessagesMessageTab::Archive,
                            _ => privatemessages::v1::GetMessagesMessageTab::Inbox,
                        }),
                    };
                    let r = privatemessages::v1::get_messages(&f.client, &p).await?;
                    let pages = r.total_pages.unwrap_or(1);
                    Ok(Page {
                        items: r
                            .collection
                            .unwrap_or_default()
                            .into_iter()
                            .map(message)
                            .collect(),
                        next: (page + 1 < pages).then(|| (page + 1).to_string()),
                    })
                }
            },
        )
    }

    /// Mark messages read or unread.
    pub fn mark_messages(&self, account: AccountId, ids: Vec<i64>, read: bool) {
        self.patch_messages(account, &ids, |m| m.read = read);
        self.inner.mutate(
            account,
            "update the message",
            "",
            &["private_messages"],
            move |f| async move {
                let body = privatemessages::v1::BatchMessagesRequest {
                    message_ids: Some(ids),
                };
                if read {
                    privatemessages::v1::post_messages_mark_read(&f.client, &body).await?;
                } else {
                    privatemessages::v1::post_messages_mark_unread(&f.client, &body).await?;
                }
                f.inner.refresh_unread(account).await;
                Ok(())
            },
        );
    }

    /// Move messages to or out of the archive.
    pub fn archive_messages(&self, account: AccountId, ids: Vec<i64>, archive: bool) {
        self.inner.mutate(
            account,
            if archive {
                "archive the message"
            } else {
                "move the message back"
            },
            if archive { "Archived." } else { "Moved to Inbox." },
            &["private_messages"],
            move |f| async move {
                let body = privatemessages::v1::BatchMessagesRequest {
                    message_ids: Some(ids),
                };
                if archive {
                    privatemessages::v1::post_messages_archive(&f.client, &body).await?;
                } else {
                    privatemessages::v1::post_messages_unarchive(&f.client, &body).await?;
                }
                Ok(())
            },
        );
    }

    fn patch_messages(&self, account: AccountId, ids: &[i64], f: impl Fn(&mut PrivateMessage) + Copy) {
        for (tab, _) in TABS {
            self.inner.res.update::<Page<PrivateMessage>>(
                &Key::private(account, "private_messages", tab),
                |p| {
                    for m in p.items.iter_mut().filter(|m| ids.contains(&m.id)) {
                        f(m);
                    }
                },
            );
        }
        self.inner.repaint();
    }

    /// The notification stream, newest first, 20 at a time.
    pub fn notifications(&self, account: AccountId, more: bool) -> Res<Page<StreamNotification>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.paged(
            Key::private(account, "notifications", ""),
            AGE,
            more,
            move |f, cursor| async move {
                let start: i32 = cursor.and_then(|c| c.parse().ok()).unwrap_or(0);
                let p = notifications::v2::GetStreamNotificationsGetRecentParams {
                    start_index: Some(start),
                    max_rows: Some(PAGE),
                };
                let r = notifications::v2::get_stream_notifications_get_recent(&f.client, &p).await?;
                let full = r.len() as i32 >= PAGE;
                Ok(Page {
                    items: r.into_iter().map(stream_entry).collect(),
                    next: full.then(|| (start + PAGE).to_string()),
                })
            },
        )
    }

    /// Clear the notification badge (what opening the bell does on the site).
    pub fn clear_notifications(&self, account: AccountId) {
        if let Some(d) = self.inner.store.write().data.get_mut(&account) {
            d.unread.notifications = 0;
        }
        self.inner.mutate(
            account,
            "clear notifications",
            "",
            &["notifications"],
            move |f| async move {
                notifications::v2::post_stream_notifications_clear_unread(&f.client).await?;
                Ok(())
            },
        );
    }
}

fn user(u: Option<privatemessages::v1::VerifiedSkinnyUserResponse>) -> (u64, String, bool) {
    let u = u.unwrap_or_default();
    (
        u.id.unwrap_or(0) as u64,
        u.display_name.or(u.name).unwrap_or_default(),
        u.has_verified_badge.unwrap_or(false),
    )
}

fn message(m: privatemessages::v1::MessageDetailsResponse) -> PrivateMessage {
    let (sender_id, sender, sender_verified) = user(m.sender);
    let (recipient_id, recipient, _) = user(m.recipient);
    PrivateMessage {
        id: m.id.unwrap_or(0),
        sender_id,
        sender,
        sender_verified,
        recipient_id,
        recipient,
        subject: m.subject.unwrap_or_default(),
        body: html_to_text(&m.body_.unwrap_or_default()),
        created: m.created.unwrap_or_default(),
        read: m.is_read.unwrap_or(true),
        system: m.is_system_message.unwrap_or(false),
    }
}

fn stream_entry(n: notifications::v2::NotificationStreamEntriesModel) -> StreamNotification {
    let content = n.content.unwrap_or_default();
    let state = content["currentState"].as_str().unwrap_or("default").to_string();
    let items = &content["states"][&state]["visualItems"];
    let body = &items["textBody"][0];
    let text = |v: &serde_json::Value| v["text"].as_str().unwrap_or_default().to_string();
    let link = items
        .as_object()
        .into_iter()
        .flat_map(|o| o.values())
        .filter_map(|v| v.as_array())
        .flatten()
        .filter_map(|i| i["actions"].as_array())
        .flatten()
        .filter_map(|a| a["path"].as_str())
        .find(|p| p.starts_with("https://") && crate::links::is_roblox_url(p))
        .map(str::to_string);
    let thumb = &items["thumbnail"][0];
    let user_id = (thumb["idType"].as_str() == Some("userAvatar")
        || thumb["idType"].as_str() == Some("user"))
    .then(|| {
        thumb["id"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .or(thumb["id"].as_u64())
    })
    .flatten();
    StreamNotification {
        id: n.id.unwrap_or_default(),
        kind: content["notificationType"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        title: text(&body["title"]),
        body: text(&body["label"]),
        when: n.event_date.unwrap_or_default(),
        read: n.is_interacted.unwrap_or(false),
        link,
        user_id,
    }
}

/// Good-enough HTML to text for message bodies: line breaks kept, tags dropped, entities decoded.
pub fn html_to_text(html: &str) -> String {
    let mut s = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(i) = rest.find('<') {
        s.push_str(&rest[..i]);
        let Some(j) = rest[i..].find('>') else {
            rest = "";
            break;
        };
        let tag = rest[i + 1..i + j].trim_start_matches('/').to_ascii_lowercase();
        let name = tag
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()
            .unwrap_or("");
        if matches!(name, "br" | "p" | "div" | "li" | "tr") {
            s.push('\n');
        }
        rest = &rest[i + j + 1..];
    }
    s.push_str(rest);
    let s = decode_entities(&s);
    let mut out = String::new();
    let mut blank = 0;
    for line in s.lines().map(str::trim_end) {
        if line.trim().is_empty() {
            blank += 1;
            if blank > 1 {
                continue;
            }
        } else {
            blank = 0;
        }
        out.push_str(line);
        out.push('\n');
    }
    out.trim().to_string()
}

fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        let end = tail.find(';').filter(|e| *e <= 10);
        let decoded = end.and_then(|e| {
            let ent = &tail[1..e];
            let c = match ent {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" | "#39" => Some('\''),
                "nbsp" => Some(' '),
                _ => ent
                    .strip_prefix("#x")
                    .and_then(|h| u32::from_str_radix(h, 16).ok())
                    .or_else(|| ent.strip_prefix('#').and_then(|d| d.parse().ok()))
                    .and_then(char::from_u32),
            };
            c.map(|c| (c, e))
        });
        match decoded {
            Some((c, e)) => {
                out.push(c);
                rest = &tail[e + 1..];
            }
            None => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_from_an_argument() {
        assert_eq!(parse_ids("5"), vec![5]);
        assert_eq!(parse_ids(" 5, 7 ,9"), vec![5, 7, 9]);
        assert_eq!(parse_ids("5,5,x,,7"), vec![5, 7]);
        assert!(parse_ids("").is_empty());
        assert!(parse_ids("abc").is_empty());
    }

    #[test]
    fn bulk_toggle_reads_unless_all_read() {
        let m = |read| PrivateMessage {
            read,
            ..Default::default()
        };
        assert!(mark_read_target(&[m(true), m(false)]));
        assert!(mark_read_target(&[m(false)]));
        assert!(!mark_read_target(&[m(true), m(true)]));
        assert!(!mark_read_target(&[]));
    }

    #[test]
    fn html_to_text_keeps_breaks_and_decodes() {
        let t = super::html_to_text(
            "<span>Hi&nbsp;there,</span><br/><br><br><p>Tom &amp; Jerry &#8212; ok&#x21;</p>",
        );
        assert_eq!(t, "Hi there,\n\nTom & Jerry \u{2014} ok!");
        assert_eq!(super::html_to_text("a & b"), "a & b");
    }
}
