//! Private messages, the notification stream and chat.
//!
//! Commands about "this conversation" read it from the page context `chat:<id>`; message
//! commands take the message id as their argument and the open tab from `messages:<tab>`.
//! `list.more` keys here are `messages:<tab>`, `notifications`, `chats` and `chat:<id>`.

use std::sync::Arc;

use bonsai_core::chat::{ConvKind, Conversation};
use bonsai_core::commands::CommandId;
use bonsai_core::messages::{self as pm, PrivateMessage};
use bonsai_core::{AccountId, Core};
use serde::Serialize;
use specta::Type;
use tauri::State;

use super::market::Paged;
use super::{GameCard, Id, Invocation, List, count};

fn signed_out<T>() -> List<T> {
    List {
        loading: false,
        error: Some("Add an account first.".to_owned()),
        items: Vec::new(),
        more: false,
    }
}

// ---------------------------------------------------------------- private messages

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MessageRow {
    pub id: Id,
    pub sender_id: Id,
    pub sender: String,
    pub sender_verified: bool,
    pub recipient_id: Id,
    pub recipient: String,
    pub subject: String,
    /// Plain text.
    pub body: String,
    /// ISO date-time.
    pub created: String,
    pub read: bool,
    /// From Roblox itself.
    pub system: bool,
}

fn message_row(m: &PrivateMessage) -> MessageRow {
    MessageRow {
        id: Id(m.id.max(0) as u64),
        sender_id: Id(m.sender_id),
        sender: m.sender.clone(),
        sender_verified: m.sender_verified,
        recipient_id: Id(m.recipient_id),
        recipient: m.recipient.clone(),
        subject: m.subject.clone(),
        body: m.body.clone(),
        created: m.created.clone(),
        read: m.read,
        system: m.system,
    }
}

/// One message tab: `inbox`, `sent`, `news` or `archive`.
#[tauri::command]
#[specta::specta]
pub fn messages(core: State<'_, Arc<Core>>, tab: String) -> Paged<MessageRow> {
    let more_key = format!("messages:{tab}");
    let Some(a) = core.acting(None) else {
        return Paged {
            list: signed_out(),
            more_key,
        };
    };
    Paged {
        list: List::paged(core.messages(a, &tab, false), message_row),
        more_key,
    }
}

/// A loaded message by id, from any tab.
fn find_message(core: &Core, a: AccountId, id: i64) -> Option<(String, PrivateMessage)> {
    bonsai_core::messages::TABS.iter().find_map(|(tab, _)| {
        let p = core.messages(a, tab, false).data?;
        let m = p.items.iter().find(|m| m.id == id)?.clone();
        Some(((*tab).to_owned(), m))
    })
}

// ---------------------------------------------------------------- notifications

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRow {
    pub id: String,
    pub title: String,
    pub body: String,
    /// ISO date-time.
    pub when: String,
    pub read: bool,
    /// A roblox.com page it points to; `entity.open_website` opens it.
    pub link: Option<String>,
    pub user_id: Option<Id>,
    /// One of [`notification_categories`]' values.
    pub category: String,
}

/// The notification categories for the filter, in display order.
#[tauri::command]
#[specta::specta]
pub fn notification_categories() -> Vec<super::market::Choice> {
    bonsai_core::messages::CATEGORIES
        .iter()
        .map(|(value, label)| super::market::Choice {
            value: (*value).to_owned(),
            label: (*label).to_owned(),
        })
        .collect()
}

#[tauri::command]
#[specta::specta]
pub fn notifications(core: State<'_, Arc<Core>>) -> Paged<NotificationRow> {
    let more_key = "notifications".to_owned();
    let Some(a) = core.acting(None) else {
        return Paged {
            list: signed_out(),
            more_key,
        };
    };
    Paged {
        list: List::paged(core.notification_feed(a), |n| NotificationRow {
            id: n.id.clone(),
            title: n.title.clone(),
            body: n.body.clone(),
            when: n.when.clone(),
            read: n.read,
            link: n.link.clone(),
            user_id: n.user_id.map(Id),
            category: n.category.clone(),
        }),
        more_key,
    }
}

// ---------------------------------------------------------------- chat

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ConversationRow {
    pub id: String,
    pub title: String,
    pub group: bool,
    /// The other person, in a one-to-one chat.
    pub partner_id: Option<Id>,
    pub unread: u32,
    /// ISO date-time of the last activity.
    pub updated: String,
    /// The last line, as the list shows it.
    pub last: String,
    pub pinned: bool,
}

fn conversation_row(c: &Conversation, me: u64, pinned: &[String]) -> ConversationRow {
    ConversationRow {
        id: c.id.clone(),
        title: c.title(me),
        group: c.kind == ConvKind::Group,
        partner_id: c.partner(me).map(Id),
        unread: count(c.unread).unwrap_or(0),
        updated: c.updated.clone(),
        last: c
            .last
            .as_ref()
            .map(|m| {
                if m.sender == me && !m.system {
                    format!("You: {}", m.text)
                } else if c.kind == ConvKind::Group && !m.system {
                    format!("{}: {}", c.name_of(m.sender).unwrap_or("Someone"), m.text)
                } else {
                    m.text.clone()
                }
            })
            .unwrap_or_default(),
        pinned: pinned.contains(&c.id),
    }
}

/// Conversations, pinned first, then newest activity.
#[tauri::command]
#[specta::specta]
pub fn chat_list(core: State<'_, Arc<Core>>) -> Paged<ConversationRow> {
    let more_key = "chats".to_owned();
    let Some(a) = core.acting(None) else {
        return Paged {
            list: signed_out(),
            more_key,
        };
    };
    let pinned = core.pinned_chats(a);
    let mut list = List::paged(core.conversations(a, false), |c| {
        conversation_row(c, a.0, &pinned)
    });
    // Stable: keeps newest-first within each group.
    list.items.sort_by_key(|c| {
        (
            !c.pinned,
            pinned.iter().position(|p| *p == c.id).unwrap_or(usize::MAX),
        )
    });
    Paged { list, more_key }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ChatLine {
    pub id: String,
    pub sender_id: Id,
    pub sender: String,
    pub mine: bool,
    pub text: String,
    /// ISO date-time.
    pub created: String,
    pub system: bool,
    /// Roblox hid it.
    pub moderated: bool,
    /// Sent from here, not confirmed yet.
    pub pending: bool,
    /// Roblox hides it from this account.
    pub hidden: bool,
    /// A note Roblox shows under the text.
    pub note: String,
    /// Sent from here and refused; `chat.resend` or `chat.discard` it.
    pub failed: bool,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ChatPerson {
    pub id: Id,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ChatView {
    pub loading: bool,
    pub error: Option<String>,
    /// `None` until the conversation list has it.
    pub info: Option<ConversationRow>,
    /// Oldest first.
    pub lines: Vec<ChatLine>,
    /// Older messages exist.
    pub more: bool,
    /// Others in it, without you.
    pub people: Vec<ChatPerson>,
    /// You started it (you can remove people).
    pub owner: bool,
    /// Names of people typing now.
    pub typing: Vec<String>,
    /// Why you can't send here, or empty.
    pub blocked: String,
    pub max_title: u32,
}

#[tauri::command]
#[specta::specta]
pub fn chat_view(core: State<'_, Arc<Core>>, conversation: String) -> ChatView {
    let empty = ChatView {
        loading: false,
        error: None,
        info: None,
        lines: Vec::new(),
        more: false,
        people: Vec::new(),
        owner: false,
        typing: Vec::new(),
        blocked: String::new(),
        max_title: 150,
    };
    let Some(a) = core.acting(None) else {
        return empty;
    };
    let conv = core
        .conversations(a, false)
        .data
        .and_then(|p| p.items.iter().find(|c| c.id == conversation).cloned());
    let name_of = |id: u64| -> String {
        if id == a.0 {
            return "You".to_owned();
        }
        conv.as_ref()
            .and_then(|c| c.name_of(id))
            .map(str::to_owned)
            .or_else(|| core.store().users.get(&id).map(|u| u.display_name.clone()))
            .unwrap_or_else(|| format!("User {id}"))
    };
    let r = core.chat_messages(a, &conversation, false);
    let settings = core.chat_settings(a).data;
    let group = conv.as_ref().is_some_and(|c| c.kind == ConvKind::Group);
    let blocked = match settings.as_deref() {
        Some(s) if !s.enabled => "Chat is off for this account.".to_owned(),
        Some(s) if group && !s.group_enabled => "Group chat is off for this account.".to_owned(),
        _ => String::new(),
    };
    let pinned = core.pinned_chats(a);
    ChatView {
        loading: r.loading,
        error: r.error,
        more: r.data.as_ref().is_some_and(|p| p.next.is_some()),
        lines: r
            .data
            .map(|p| p.items.iter().rev().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
            .into_iter()
            .chain(core.failed_chat(a, &conversation))
            .map(|m| ChatLine {
                id: m.id.clone(),
                sender_id: Id(m.sender),
                sender: name_of(m.sender),
                mine: m.sender == a.0,
                text: m.text,
                created: m.created,
                system: m.system,
                moderated: m.moderated,
                pending: m.pending,
                hidden: m.hidden,
                note: m.note,
                failed: m.failed,
            })
            .collect(),
        people: conv
            .as_ref()
            .map(|c| {
                c.others(a.0)
                    .into_iter()
                    .map(|id| ChatPerson {
                        id: Id(id),
                        name: name_of(id),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        owner: conv.as_ref().is_some_and(|c| c.created_by == a.0),
        typing: core
            .typing_in(a, &conversation)
            .into_iter()
            .map(name_of)
            .collect(),
        blocked,
        max_title: settings.map_or(150, |s| s.max_title as u32),
        info: conv.as_ref().map(|c| conversation_row(c, a.0, &pinned)),
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ChatOptions {
    pub loading: bool,
    pub error: Option<String>,
    pub enabled: bool,
    pub group_enabled: bool,
    pub max_title: u32,
}

/// Account-specific permissions and limits for the new chat dialog.
#[tauri::command]
#[specta::specta]
pub fn chat_options(core: State<'_, Arc<Core>>) -> ChatOptions {
    let settings = core.acting(None).map(|a| core.chat_settings(a));
    let data = settings.as_ref().and_then(|s| s.data.as_deref());
    ChatOptions {
        loading: settings.as_ref().is_some_and(|s| s.loading),
        error: settings.as_ref().and_then(|s| s.error.clone()),
        enabled: data.is_some_and(|s| s.enabled),
        group_enabled: data.is_some_and(|s| s.enabled && s.group_enabled),
        max_title: data.map_or(150, |s| s.max_title as u32),
    }
}

/// A conversation the chat screen should open, once (after starting a chat).
#[tauri::command]
#[specta::specta]
pub fn take_chat_focus(core: State<'_, Arc<Core>>) -> Option<String> {
    core.take_chat_focus(core.acting(None)?)
}

/// The card for an experience linked from a chat line or message, by place id. `None` until
/// the experience is known; asking starts the fetch.
#[tauri::command]
#[specta::specta]
pub fn linked_game(core: State<'_, Arc<Core>>, place_id: Id) -> Option<GameCard> {
    let account = core.acting(None).unwrap_or(AccountId::ANONYMOUS);
    core.ensure_place(account, place_id.0);
    let universe = core.store().place_universe.get(&place_id.0).copied()?;
    let store = core.store();
    store.games.get(&universe).map(GameCard::from)
}

/// Marks a conversation read. Opening it does this.
#[tauri::command]
#[specta::specta]
pub fn mark_chat_read(core: State<'_, Arc<Core>>, conversation: String) {
    if let Some(a) = core.acting(None) {
        core.mark_chat_read(a, &conversation);
    }
}

/// Tells the others you're typing. Keystrokes in the composer call this; it throttles itself.
#[tauri::command]
#[specta::specta]
pub fn chat_typing(core: State<'_, Arc<Core>>, conversation: String) {
    if let Some(a) = core.acting(None) {
        core.chat_typing(a, &conversation);
    }
}

/// Starts a chat with these people (a group when more than one). Called only from the dialog
/// `chat.new_group` opens; the chat screen opens it when it's ready.
#[tauri::command]
#[specta::specta]
pub fn start_chat(core: State<'_, Arc<Core>>, users: Vec<Id>, name: Option<String>) -> Result<(), String> {
    let a = core
        .acting(None)
        .ok_or_else(|| "Add an account first.".to_owned())?;
    if users.is_empty() {
        return Err("Pick someone to chat with.".to_owned());
    }
    core.start_chat(a, users.into_iter().map(|u| u.0).collect(), name);
    Ok(())
}

// ---------------------------------------------------------------- commands

/// Runs a `message.*`, `notifications.*`, `chat.*` or `user.message` command, or a `list.more`
/// for one of these lists. `None` for anything else.
pub(super) fn run(
    core: &Core,
    id: CommandId,
    inv: &Invocation,
    account: Option<AccountId>,
) -> Option<Result<(), String>> {
    use CommandId as C;
    let need = || account.ok_or_else(|| "Add an account first.".to_owned());
    let arg = inv.arg.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let chat = || {
        inv.context
            .as_deref()
            .and_then(|c| c.strip_prefix("chat:"))
            .filter(|c| !c.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| "Open a conversation first.".to_owned())
    };
    // The argument is one message id, or several with commas (bulk actions).
    let selected = |a: AccountId| {
        let ids = arg.map(pm::parse_ids).unwrap_or_default();
        if ids.is_empty() {
            return Err("Which message?".to_owned());
        }
        let found: Vec<(String, PrivateMessage)> =
            ids.iter().filter_map(|id| find_message(core, a, *id)).collect();
        if found.is_empty() {
            Err("That message isn't loaded.".to_owned())
        } else {
            Ok(found)
        }
    };
    let message = |a: AccountId| {
        selected(a).and_then(|v| v.into_iter().next().ok_or_else(|| "Which message?".to_owned()))
    };
    Some(match id {
        C::MessageUser => (|| {
            let a = need()?;
            let who = arg
                .map(str::to_owned)
                .or_else(|| {
                    inv.context
                        .as_deref()
                        .filter(|c| c.starts_with("user:"))
                        .and_then(bonsai_core::links::target_id)
                        .map(|id| id.to_string())
                })
                .ok_or_else(|| "Message whom?".to_owned())?;
            core.with_user(&who, move |c, user| {
                if user == a.0 {
                    c.notice("That's you.", true);
                } else {
                    c.start_chat(a, vec![user], None);
                }
            })
        })(),
        C::ReplyMessage => (|| {
            let a = need()?;
            let (_, m) = message(a)?;
            if m.system || m.sender_id == 0 {
                return Err("Roblox messages can't be answered.".to_owned());
            }
            // Roblox no longer takes new private messages; replies go to chat.
            core.start_chat(a, vec![m.sender_id], None);
            Ok(())
        })(),
        C::ToggleMessageRead => (|| {
            let a = need()?;
            let found = selected(a)?;
            let all: Vec<PrivateMessage> = found.into_iter().map(|(_, m)| m).collect();
            let read = pm::mark_read_target(&all);
            core.mark_messages(a, all.iter().map(|m| m.id).collect(), read);
            Ok(())
        })(),
        C::ArchiveMessage => (|| {
            let a = need()?;
            let found = selected(a)?;
            // A selection is from one tab; the first message decides which way it goes.
            let tab = found[0].0.clone();
            if !matches!(tab.as_str(), "inbox" | "archive") {
                return Err("Only Inbox messages can be archived.".to_owned());
            }
            let ids = found
                .iter()
                .filter(|(t, _)| *t == tab)
                .map(|(_, m)| m.id)
                .collect();
            core.archive_messages(a, ids, tab != "archive");
            Ok(())
        })(),
        // Handled by the Messages screen, which owns the selection.
        C::SelectAllMessages => Err("Open Messages first.".to_owned()),
        C::ChatMembers => Err("Open a group chat first.".to_owned()),
        C::ChatEmoji => Err("Open a conversation first.".to_owned()),
        C::ShareExperience => (|| {
            let a = need()?;
            let conv = chat()?;
            let place = arg
                .and_then(bonsai_core::links::shared_place)
                .ok_or_else(|| "Give an experience link or ID.".to_owned())?;
            let url = bonsai_core::links::page(&format!("game:{place}"), None)
                .ok_or_else(|| "Can't make a link to that experience.".to_owned())?;
            core.send_chat(a, &conv, &url);
            Ok(())
        })(),
        C::MarkAllMessagesRead => (|| {
            let a = need()?;
            let unread: Vec<i64> = core
                .messages(a, "inbox", false)
                .data
                .map(|p| p.items.iter().filter(|m| !m.read).map(|m| m.id).collect())
                .unwrap_or_default();
            if unread.is_empty() {
                return Err("Nothing unread.".to_owned());
            }
            core.mark_messages(a, unread, true);
            Ok(())
        })(),
        C::ClearNotifications => (|| {
            core.clear_notifications(need()?);
            Ok(())
        })(),
        C::SendChat => (|| {
            let text = arg.ok_or_else(|| "Write something.".to_owned())?;
            core.send_chat(need()?, &chat()?, text);
            Ok(())
        })(),
        C::ResendChat => (|| {
            let id = arg.ok_or_else(|| "Which message?".to_owned())?;
            if core.resend_chat(need()?, &chat()?, id) {
                Ok(())
            } else {
                Err("No failed message with that id here.".to_owned())
            }
        })(),
        C::DiscardChat => (|| {
            let id = arg.ok_or_else(|| "Which message?".to_owned())?;
            if core.discard_chat(need()?, &chat()?, id) {
                Ok(())
            } else {
                Err("No failed message with that id here.".to_owned())
            }
        })(),
        C::RenameChat => (|| {
            let name = arg.ok_or_else(|| "Give a name.".to_owned())?;
            core.rename_chat(need()?, &chat()?, name);
            Ok(())
        })(),
        C::AddToChat => (|| {
            let a = need()?;
            let conv = chat()?;
            let who = arg.ok_or_else(|| "Add whom?".to_owned())?;
            for w in who.split(',').map(str::trim).filter(|w| !w.is_empty()) {
                let conv = conv.clone();
                core.with_user(w, move |c, user| c.add_to_chat(a, &conv, vec![user]))?;
            }
            Ok(())
        })(),
        C::RemoveFromChat => (|| {
            let a = need()?;
            let conv = chat()?;
            let who = arg.ok_or_else(|| "Remove whom?".to_owned())?;
            core.with_user(who, move |c, user| c.remove_from_chat(a, &conv, user))
        })(),
        C::LeaveChat => (|| {
            core.leave_chat(need()?, &chat()?);
            Ok(())
        })(),
        C::PinChat => (|| {
            core.toggle_pin_chat(need()?, &chat()?);
            Ok(())
        })(),
        C::NewGroupChat => Err("Open Chat to start a conversation.".to_owned()),
        C::LoadMore => {
            let key = inv.arg.as_deref()?.trim();
            let a = account?;
            let (what, rest) = key.split_once(':').unwrap_or((key, ""));
            match what {
                "messages" => drop(core.messages(a, rest, true)),
                "notifications" => drop(core.notifications(a, true)),
                "chats" => drop(core.conversations(a, true)),
                "chat" => drop(core.chat_messages(a, rest, true)),
                _ => return None,
            }
            Ok(())
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bonsai_core::chat::ChatMessage;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn group_preview_identifies_the_sender() {
        let mut c = Conversation {
            kind: ConvKind::Group,
            names: vec![(2, "Ann".into())],
            last: Some(ChatMessage {
                sender: 2,
                text: "hello".into(),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(conversation_row(&c, 1, &[]).last, "Ann: hello");
        assert_eq!(conversation_row(&c, 2, &[]).last, "You: hello");
        c.last.as_mut().unwrap().system = true;
        assert_eq!(conversation_row(&c, 1, &[]).last, "hello");
        c.last.as_mut().unwrap().system = false;
        c.kind = ConvKind::OneToOne;
        assert_eq!(conversation_row(&c, 1, &[]).last, "hello");
    }
}
