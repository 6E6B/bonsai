//! Chat (platform-chat-api) and the realtime hub that keeps it, the unread badges and the
//! friend requests live.
//!
//! Reads are cached resources; writes are user actions (one click or keypress each). The
//! realtime loop only listens: events invalidate caches and update typing state, they never
//! send anything.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use rbx_api::generated::apis;
use rbx_api::realtime::Event;
use rbx_api::{AccountId, Error};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::cache::{Kind, now};
use crate::core::Inner;
use crate::res::{Key, Page, Res};
use crate::store::UserInfo;

const LIST_AGE: i64 = 60;
const MESSAGES_AGE: i64 = 60;
const SETTINGS_AGE: i64 = 3600;
const PAGE: i32 = 30;
/// Typing status goes out at most this often per conversation.
const TYPING_EVERY_MS: i64 = 3000;
/// Someone counts as typing for this long after their last typing event.
const TYPING_FOR_MS: i64 = 6000;
/// The site's limit on one chat message.
pub const MAX_MESSAGE_CHARS: usize = 400;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvKind {
    #[default]
    OneToOne,
    Group,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub kind: ConvKind,
    /// The group's title, or empty for one-to-one and untitled groups (see [`Conversation::title`]).
    pub name: String,
    pub participants: Vec<u64>,
    pub created_by: u64,
    pub unread: i64,
    /// ISO date-time of the last activity.
    pub updated: String,
    pub last: Option<ChatMessage>,
    /// Display names of the participants, by id (from the conversation's user data).
    pub names: Vec<(u64, String)>,
}

impl Conversation {
    /// The other people in it, without `me`.
    pub fn others(&self, me: u64) -> Vec<u64> {
        self.participants.iter().copied().filter(|p| *p != me).collect()
    }

    /// The person a one-to-one chat is with.
    pub fn partner(&self, me: u64) -> Option<u64> {
        (self.kind == ConvKind::OneToOne)
            .then(|| self.others(me).first().copied())
            .flatten()
    }

    pub fn name_of(&self, id: u64) -> Option<&str> {
        self.names.iter().find(|(i, _)| *i == id).map(|(_, n)| n.as_str())
    }

    /// What the list shows: the group's name, the partner, or "A, B and 3 others".
    pub fn title(&self, me: u64) -> String {
        if !self.name.trim().is_empty() {
            return self.name.clone();
        }
        let others: Vec<String> = self
            .others(me)
            .into_iter()
            .map(|id| {
                self.name_of(id)
                    .map_or_else(|| format!("User {id}"), str::to_string)
            })
            .collect();
        match others.len() {
            0 => "Just you".into(),
            1 | 2 => others.join(" and "),
            n => format!("{}, {} and {} others", others[0], others[1], n - 2),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub sender: u64,
    pub text: String,
    /// ISO date-time.
    pub created: String,
    /// "X joined the chat" and similar lines from Roblox.
    pub system: bool,
    /// Roblox hid it (filtered or moderated).
    pub moderated: bool,
    /// Sent from here and not confirmed yet.
    pub pending: bool,
}

/// What the account may do in chat.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatSettings {
    pub enabled: bool,
    pub group_enabled: bool,
    pub max_title: usize,
}

fn conversation(c: apis::undocumented::ChatConversation) -> Option<Conversation> {
    let mut names: Vec<(u64, String)> = c
        .user_data
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(k, u)| {
            let id = k.parse::<u64>().ok().or(u.id.map(|i| i as u64))?;
            let n = u.display_name.or(u.combined_name).or(u.name)?;
            Some((id, n))
        })
        .collect();
    names.sort();
    Some(Conversation {
        id: c.id.filter(|i| !i.is_empty())?,
        kind: if c.type_.as_deref() == Some("group") {
            ConvKind::Group
        } else {
            ConvKind::OneToOne
        },
        name: c.name.unwrap_or_default(),
        participants: c
            .participant_user_ids
            .unwrap_or_default()
            .into_iter()
            .filter(|i| *i > 0)
            .map(|i| i as u64)
            .collect(),
        created_by: c.created_by.unwrap_or(0) as u64,
        unread: c.unread_message_count.unwrap_or(0) as i64,
        updated: c.updated_at.unwrap_or_default(),
        last: c.messages.unwrap_or_default().into_iter().find_map(message),
        names,
    })
}

fn message(m: apis::undocumented::ChatMessage) -> Option<ChatMessage> {
    let text = m.content.unwrap_or_default();
    // Roblox sets `moderation_type` on ordinary messages too, so it only means
    // hidden when the content is gone.
    let moderated = m.status.as_deref() == Some("moderated")
        || (text.is_empty() && m.moderation_type.is_some_and(|t| !t.is_empty() && t != "none"));
    Some(ChatMessage {
        id: m.id.filter(|i| !i.is_empty())?,
        sender: m.sender_user_id.unwrap_or(0).max(0) as u64,
        text,
        created: m.created_at.unwrap_or_default(),
        system: m.type_.as_deref() == Some("system"),
        moderated,
        pending: false,
    })
}

fn conversation_users(c: &Conversation) -> Vec<UserInfo> {
    c.names
        .iter()
        .map(|(id, n)| UserInfo {
            id: *id,
            name: String::new(),
            display_name: n.clone(),
            verified: false,
        })
        .collect()
}

impl Core {
    /// Conversations, newest activity first.
    pub fn conversations(&self, account: AccountId, more: bool) -> Res<Page<Conversation>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.paged(
            Key::private(account, "chat_conversations", ""),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = apis::undocumented::GetChatConversationsParams {
                    include_user_data: Some(true),
                    cursor,
                    page_size: Some(PAGE),
                };
                let r = apis::undocumented::get_chat_conversations(&f.client, &p).await?;
                let items: Vec<Conversation> = r
                    .conversations
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(conversation)
                    .collect();
                // Only names we don't have: user data has display names but no @names.
                {
                    let s = f.inner.store.read();
                    let missing: Vec<UserInfo> = items
                        .iter()
                        .flat_map(conversation_users)
                        .filter(|u| !s.users.contains_key(&u.id))
                        .collect();
                    drop(s);
                    f.note_users(missing);
                }
                Ok(Page {
                    items,
                    next: r.next_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// Messages of one conversation, newest first (the screen draws them oldest at the top).
    pub fn chat_messages(
        &self,
        account: AccountId,
        conversation: &str,
        more: bool,
    ) -> Res<Page<ChatMessage>> {
        if account.is_anonymous() || conversation.is_empty() {
            return Res::default();
        }
        let conv = conversation.to_string();
        self.inner.paged(
            Key::private(account, "chat_messages", &conv),
            MESSAGES_AGE,
            more,
            move |f, cursor| {
                let conv = conv.clone();
                async move {
                    let p = apis::undocumented::GetChatMessagesParams {
                        conversation_id: conv,
                        cursor,
                        page_size: Some(PAGE),
                    };
                    let r = apis::undocumented::get_chat_messages(&f.client, &p).await?;
                    Ok(Page {
                        items: r
                            .messages
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(message)
                            .collect(),
                        next: r.next_cursor.filter(|c| !c.is_empty()),
                    })
                }
            },
        )
    }

    pub fn chat_settings(&self, account: AccountId) -> Res<ChatSettings> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "chat_settings", ""),
            SETTINGS_AGE,
            move |f| async move {
                let r = apis::undocumented::get_chat_settings(&f.client).await?;
                let on = |s: &Option<String>| s.as_deref().is_none_or(|v| v != "disabled");
                Ok(ChatSettings {
                    enabled: r.is_chat_enabled.unwrap_or(true) && on(&r.is_chat_enabled_by_privacy_setting),
                    group_enabled: on(&r.is_group_chat_enabled_by_privacy_setting),
                    max_title: r.max_conversation_title_length.unwrap_or(150).max(1) as usize,
                })
            },
        )
    }

    /// Send a message (Enter in the composer). Shows at once; the server copy replaces it.
    pub fn send_chat(&self, account: AccountId, conversation: &str, text: &str) {
        let text: String = text.trim().chars().take(MAX_MESSAGE_CHARS).collect();
        if text.is_empty() || conversation.is_empty() {
            return;
        }
        let conv = conversation.to_string();
        let local = ChatMessage {
            id: format!("local-{}", now_ms()),
            sender: account.0,
            text: text.clone(),
            created: crate::fmt::iso_utc(now()),
            pending: true,
            ..Default::default()
        };
        self.inner
            .res
            .update::<Page<ChatMessage>>(&Key::private(account, "chat_messages", &conv), |p| {
                p.items.insert(0, local.clone())
            });
        self.patch_conversation(account, &conv, |c| {
            c.last = Some(local.clone());
            c.updated = local.created.clone();
        });
        self.inner.repaint();
        self.inner.mutate(
            account,
            "send the message",
            "",
            &["chat_messages", "chat_conversations"],
            move |f| async move {
                let body = apis::undocumented::ChatSendRequest {
                    conversation_id: Some(conv),
                    messages: Some(vec![apis::undocumented::ChatOutgoing { content: Some(text) }]),
                };
                let r = apis::undocumented::post_chat_send(&f.client, &body).await?;
                if r.messages
                    .unwrap_or_default()
                    .iter()
                    .any(|m| m.status.as_deref() == Some("moderated"))
                {
                    return Err(Error::Message(
                        "Roblox didn't deliver it because of its chat filter.".into(),
                    ));
                }
                Ok(())
            },
        );
    }

    /// Tell the others you're typing. Called on keystrokes; sent at most every few seconds.
    pub fn chat_typing(&self, account: AccountId, conversation: &str) {
        if account.is_anonymous() || conversation.is_empty() {
            return;
        }
        let key = format!("typing:{}:{conversation}", account.0);
        let t = now_ms();
        {
            let mut th = self.inner.throttle.lock();
            if th.get(&key).is_some_and(|last| t - last < TYPING_EVERY_MS) {
                return;
            }
            th.insert(key, t);
        }
        let Some(c) = self.inner.client(account).filter(|c| c.signed_in()) else {
            return;
        };
        let body = apis::undocumented::ChatConversationRef {
            conversation_id: Some(conversation.to_string()),
        };
        self.inner.rt.spawn(async move {
            let _ = apis::undocumented::post_chat_typing(&c, &body).await;
        });
    }

    /// People typing in a conversation right now (not you).
    pub fn typing_in(&self, account: AccountId, conversation: &str) -> Vec<u64> {
        let t = now_ms();
        let s = self.inner.store.read();
        let mut v: Vec<u64> = s
            .account_data(account)
            .and_then(|d| d.typing.get(conversation))
            .map(|m| {
                m.iter()
                    .filter(|(id, until)| **until > t && **id != account.0)
                    .map(|(id, _)| *id)
                    .collect()
            })
            .unwrap_or_default();
        v.sort();
        v
    }

    /// Mark a conversation read (opening it does this).
    pub fn mark_chat_read(&self, account: AccountId, conversation: &str) {
        let mut was_unread = false;
        self.patch_conversation(account, conversation, |c| {
            was_unread = c.unread > 0;
            c.unread = 0;
        });
        if !was_unread {
            return;
        }
        if let Some(d) = self.inner.store.write().data.get_mut(&account) {
            d.unread.chat = (d.unread.chat - 1).max(0);
        }
        let conv = conversation.to_string();
        self.inner
            .mutate(account, "mark the chat read", "", &[], move |f| async move {
                let body = apis::undocumented::ChatMarkRequest {
                    conversation_ids: Some(vec![conv]),
                };
                apis::undocumented::post_chat_mark_read(&f.client, &body).await?;
                f.inner.refresh_unread(account).await;
                Ok(())
            });
    }

    /// Open a chat with these people: an existing one-to-one chat if there is one, else a new
    /// conversation. The screen opens it when [`Core::take_chat_focus`] returns its id.
    pub fn start_chat(&self, account: AccountId, users: Vec<u64>, name: Option<String>) {
        let users: Vec<u64> = {
            let mut seen = HashSet::new();
            users
                .into_iter()
                .filter(|u| *u > 0 && *u != account.0 && seen.insert(*u))
                .collect()
        };
        if users.is_empty() {
            return;
        }
        if let [one] = users.as_slice()
            && let Some(c) = self.conversations(account, false).data.and_then(|p| {
                p.items
                    .iter()
                    .find(|c| c.partner(account.0) == Some(*one))
                    .map(|c| c.id.clone())
            })
        {
            self.set_chat_focus(account, c);
            return;
        }
        let group = users.len() > 1;
        let inner = self.inner.clone();
        self.inner.mutate(
            account,
            "start the chat",
            "",
            &["chat_conversations"],
            move |f| async move {
                let body = apis::undocumented::ChatCreateRequest {
                    conversations: Some(vec![apis::undocumented::ChatNewConversation {
                        type_: Some(if group { "group" } else { "one_to_one" }.into()),
                        name: name.filter(|n| !n.trim().is_empty() && group),
                        participant_user_ids: Some(users.iter().map(|u| *u as i64).collect()),
                    }]),
                    include_user_data: Some(true),
                };
                let r = apis::undocumented::post_chat_create(&f.client, &body).await?;
                let id = r
                    .conversations
                    .unwrap_or_default()
                    .into_iter()
                    .find_map(|c| c.id)
                    .ok_or_else(|| Error::Message("Roblox didn't start the chat.".into()))?;
                inner.store.write().data.entry(account).or_default().chat_focus = Some(id);
                Ok(())
            },
        );
    }

    /// A conversation the chat screen should open, once.
    pub fn take_chat_focus(&self, account: AccountId) -> Option<String> {
        self.inner
            .store
            .write()
            .data
            .get_mut(&account)
            .and_then(|d| d.chat_focus.take())
    }

    fn set_chat_focus(&self, account: AccountId, id: String) {
        self.inner
            .store
            .write()
            .data
            .entry(account)
            .or_default()
            .chat_focus = Some(id);
        self.inner.repaint();
    }

    pub fn rename_chat(&self, account: AccountId, conversation: &str, name: &str) {
        let (conv, name) = (conversation.to_string(), name.trim().to_string());
        let n = name.clone();
        self.patch_conversation(account, conversation, |c| c.name = n.clone());
        self.inner.mutate(
            account,
            "rename the chat",
            "Renamed",
            &["chat_conversations"],
            move |f| async move {
                let body = apis::undocumented::ChatUpdateRequest {
                    conversations: Some(vec![apis::undocumented::ChatRename {
                        id: Some(conv),
                        name: Some(name),
                    }]),
                };
                apis::undocumented::post_chat_update(&f.client, &body).await?;
                Ok(())
            },
        );
    }

    pub fn add_to_chat(&self, account: AccountId, conversation: &str, users: Vec<u64>) {
        if users.is_empty() {
            return;
        }
        self.chat_users(account, conversation, users, true);
    }

    pub fn remove_from_chat(&self, account: AccountId, conversation: &str, user: u64) {
        self.chat_users(account, conversation, vec![user], false);
    }

    /// Leave a group chat (remove yourself).
    pub fn leave_chat(&self, account: AccountId, conversation: &str) {
        self.inner
            .res
            .update::<Page<Conversation>>(&Key::private(account, "chat_conversations", ""), |p| {
                p.items.retain(|c| c.id != conversation)
            });
        self.chat_users(account, conversation, vec![account.0], false);
    }

    fn chat_users(&self, account: AccountId, conversation: &str, users: Vec<u64>, add: bool) {
        let conv = conversation.to_string();
        let leaving = !add && users == [account.0];
        self.inner.mutate(
            account,
            match (add, leaving) {
                (true, _) => "add people",
                (false, true) => "leave the chat",
                (false, false) => "remove them",
            },
            match (add, leaving) {
                (true, _) => "Added to the chat",
                (false, true) => "You left the chat",
                (false, false) => "Removed from the chat",
            },
            &["chat_conversations", "chat_messages"],
            move |f| async move {
                let body = apis::undocumented::ChatUsersRequest {
                    conversation_id: Some(conv),
                    user_ids: Some(users.iter().map(|u| *u as i64).collect()),
                };
                if add {
                    apis::undocumented::post_chat_add_users(&f.client, &body).await?;
                } else {
                    apis::undocumented::post_chat_remove_users(&f.client, &body).await?;
                }
                Ok(())
            },
        );
    }

    fn patch_conversation(&self, account: AccountId, id: &str, f: impl FnOnce(&mut Conversation)) {
        self.inner
            .res
            .update::<Page<Conversation>>(&Key::private(account, "chat_conversations", ""), |p| {
                if let Some(c) = p.items.iter_mut().find(|c| c.id == id) {
                    f(c);
                }
            });
    }

    /// Conversations pinned to the top of the list (local; Roblox has no pins).
    pub fn pinned_chats(&self, account: AccountId) -> Vec<String> {
        self.local::<Vec<String>>(account, "pinned_chats")
    }

    pub fn toggle_pin_chat(&self, account: AccountId, conversation: &str) {
        let mut v = self.pinned_chats(account);
        if let Some(i) = v.iter().position(|c| c == conversation) {
            v.remove(i);
        } else {
            v.insert(0, conversation.to_string());
        }
        self.set_local(account, "pinned_chats", &v);
    }

    /// Feed one realtime event (the loop does this; tests call it directly).
    pub fn apply_realtime(&self, e: &Event) {
        self.inner.on_realtime(e);
    }

    pub(crate) fn local<T: serde::de::DeserializeOwned + Default>(&self, account: AccountId, id: &str) -> T {
        self.inner
            .cache
            .get::<T>(Kind::Local, account, id)
            .map(|c| c.value)
            .unwrap_or_default()
    }

    pub(crate) fn set_local<T: Serialize>(&self, account: AccountId, id: &str, v: &T) {
        self.inner.cache.put(Kind::Local, account, id, v);
        self.inner.repaint();
    }
}

impl Inner {
    fn on_realtime(self: &Arc<Self>, e: &Event) {
        let acc = e.account;
        let kind = e.kind();
        let field = |k: &str| {
            e.detail.get(k).and_then(|v| {
                v.as_str()
                    .map(str::to_string)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            })
        };
        match e.namespace.as_str() {
            "ChatNotifications" => {
                let conv = field("ConversationId").unwrap_or_default();
                if kind == "participanttyping" {
                    let user = field("UserId").and_then(|u| u.parse::<u64>().ok()).unwrap_or(0);
                    let on = e.detail.get("IsTyping").and_then(|v| v.as_bool()).unwrap_or(true);
                    let mut s = self.store.write();
                    let m = s.data.entry(acc).or_default().typing.entry(conv).or_default();
                    if on {
                        m.insert(user, now_ms() + TYPING_FOR_MS);
                    } else {
                        m.remove(&user);
                    }
                    drop(s);
                    // Repaint again when the indicator should go away.
                    let me = self.clone();
                    self.rt.spawn(async move {
                        tokio::time::sleep(Duration::from_millis(TYPING_FOR_MS as u64 + 50)).await;
                        me.repaint();
                    });
                } else {
                    if kind == "newmessage"
                        && let Some(sender) = field("SenderUserId").and_then(|u| u.parse::<u64>().ok())
                        && let Some(d) = self.store.write().data.get_mut(&acc)
                        && let Some(m) = d.typing.get_mut(&conv)
                    {
                        m.remove(&sender);
                    }
                    self.res.invalidate(acc, "chat_conversations");
                    self.res.invalidate(acc, "chat_messages");
                    self.spawn_unread(acc);
                }
            }
            "NotificationStream" => {
                self.res.invalidate(acc, "notifications");
                self.spawn_unread(acc);
            }
            "MessageNotification" | "PrivateMessageNotifications" => {
                self.res.invalidate(acc, "private_messages");
                self.spawn_unread(acc);
            }
            "FriendshipNotifications" => {
                self.res.invalidate(acc, "friend_requests");
                self.res.invalidate(acc, "relation");
                self.spawn_unread(acc);
                // Someone added or removed you: re-read the list so a removal is noticed now.
                if kind.starts_with("friendshipdestroyed") || kind.starts_with("friendshipcreated") {
                    let me = self.clone();
                    self.rt.spawn(async move { me.refresh_friends(acc).await });
                }
            }
            _ => {}
        }
        self.repaint();
    }

    fn spawn_unread(self: &Arc<Self>, acc: AccountId) {
        let me = self.clone();
        self.rt.spawn(async move { me.refresh_unread(acc).await });
    }
}

/// Keep one realtime connection per signed-in account (all of them, or only the active one
/// when "keep other accounts live" is off), reconnecting with backoff.
pub(crate) async fn realtime_loop(inner: Arc<Inner>) {
    let mut running: HashMap<AccountId, tokio::task::JoinHandle<()>> = HashMap::new();
    let mut tick = tokio::time::interval(Duration::from_secs(10));
    loop {
        tick.tick().await;
        let want: HashSet<AccountId> = {
            let s = inner.store.read();
            let all = inner.settings.read().background_accounts;
            s.accounts
                .accounts
                .iter()
                .map(|a| a.id)
                .filter(|id| all || Some(*id) == s.accounts.active)
                .filter(|id| {
                    s.account_data(*id)
                        .is_none_or(|d| d.session != crate::store::SessionState::Expired)
                })
                .collect()
        };
        running.retain(|id, h| {
            let keep = want.contains(id) && !h.is_finished();
            if !keep {
                h.abort();
            }
            keep
        });
        for id in want {
            if running.contains_key(&id) {
                continue;
            }
            let me = inner.clone();
            running.insert(id, inner.rt.spawn(connection(me, id)));
        }
    }
}

async fn connection(inner: Arc<Inner>, id: AccountId) {
    let mut backoff = 2u64;
    loop {
        let Some(c) = inner.client(id).filter(|c| c.signed_in()) else {
            return;
        };
        let me = inner.clone();
        let started = std::time::Instant::now();
        set_live(&inner, id, true);
        let r = rbx_api::realtime::run(&c, move |e| me.on_realtime(&e)).await;
        set_live(&inner, id, false);
        if let Err(e) = &r {
            inner.note_error(id, e);
            if matches!(e, Error::Unauthorized) {
                return;
            }
        }
        // A connection that lasted a while resets the backoff.
        if started.elapsed() > Duration::from_secs(120) {
            backoff = 2;
        }
        let jitter = fastrand::u64(0..1000);
        tokio::time::sleep(Duration::from_millis(backoff * 1000 + jitter)).await;
        backoff = (backoff * 2).min(60);
    }
}

fn set_live(inner: &Inner, id: AccountId, on: bool) {
    inner.store.write().data.entry(id).or_default().realtime = on;
    inner.repaint();
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conv(name: &str, me: u64, others: &[(u64, &str)]) -> Conversation {
        let mut participants = vec![me];
        participants.extend(others.iter().map(|o| o.0));
        Conversation {
            id: "c".into(),
            kind: if others.len() > 1 {
                ConvKind::Group
            } else {
                ConvKind::OneToOne
            },
            name: name.into(),
            participants,
            names: others.iter().map(|(i, n)| (*i, n.to_string())).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn titles() {
        assert_eq!(conv("", 1, &[(2, "Ann")]).title(1), "Ann");
        assert_eq!(conv("", 1, &[(2, "Ann")]).partner(1), Some(2));
        assert_eq!(conv("", 1, &[(2, "Ann"), (3, "Bo")]).title(1), "Ann and Bo");
        assert_eq!(
            conv("", 1, &[(2, "Ann"), (3, "Bo"), (4, "Cy"), (5, "Di")]).title(1),
            "Ann, Bo and 2 others"
        );
        assert_eq!(conv("Squad", 1, &[(2, "Ann"), (3, "Bo")]).title(1), "Squad");
        assert_eq!(conv("", 1, &[(2, "Ann"), (3, "Bo")]).partner(1), None);
        assert_eq!(conv("", 1, &[(9, "")]).others(1), vec![9]);
    }

    #[test]
    fn moderated_and_system_messages() {
        let m = message(apis::undocumented::ChatMessage {
            id: Some("m".into()),
            status: Some("moderated".into()),
            type_: Some("system".into()),
            ..Default::default()
        })
        .unwrap();
        assert!(m.moderated && m.system);
        let ok = message(apis::undocumented::ChatMessage {
            id: Some("m".into()),
            content: Some("hi".into()),
            status: Some("success".into()),
            moderation_type: Some("unmoderated".into()),
            ..Default::default()
        })
        .unwrap();
        assert!(!ok.moderated, "moderation_type alone does not hide text");
        assert!(
            message(apis::undocumented::ChatMessage::default()).is_none(),
            "no id, no message"
        );
    }
}
