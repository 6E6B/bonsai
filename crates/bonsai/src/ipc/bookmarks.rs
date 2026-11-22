//! Bookmark command forwarding and cached row views. Never schedules entity fetches.

use super::{Id, Invocation, viewer};
use bonsai_core::{
    AccountId, Core,
    bookmarks::{self, BookmarkKind},
    commands::CommandId,
    fmt,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum BookmarkKindView {
    Experience,
    Person,
    Item,
    Community,
    View,
    Unknown,
}
impl From<BookmarkKind> for BookmarkKindView {
    fn from(kind: BookmarkKind) -> Self {
        match kind {
            BookmarkKind::Experience => Self::Experience,
            BookmarkKind::Person => Self::Person,
            BookmarkKind::Item => Self::Item,
            BookmarkKind::Community => Self::Community,
            BookmarkKind::View => Self::View,
            BookmarkKind::Unknown => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkThumbnailKind {
    Headshot,
    GameIcon,
    Asset,
    Bundle,
    GamePass,
    GroupIcon,
}
#[derive(Clone, Debug, Serialize, Type)]
pub struct BookmarkThumbnail {
    pub kind: BookmarkThumbnailKind,
    pub id: Id,
}
#[derive(Clone, Debug, Serialize, Type)]
pub struct BookmarkView {
    pub id: Id,
    pub path: String,
    pub title: String,
    pub kind: BookmarkKindView,
    pub thumbnail: Option<BookmarkThumbnail>,
    pub status: String,
    /// Empty outside folders.
    pub folder: String,
    pub tags: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Type)]
pub struct BookmarkList {
    pub account: Id,
    pub items: Vec<BookmarkView>,
}

fn details(core: &Core, account: AccountId, path: &str) -> (String, Option<BookmarkThumbnail>, String) {
    let Some((route, id)) = bookmarks::entity(path) else {
        return (String::new(), None, String::new());
    };
    let s = core.store();
    let thumb = |kind, id| Some(BookmarkThumbnail { kind, id: Id(id) });
    match route {
        "games" => {
            let game = s.game_for_place(id);
            (
                game.map(|g| g.name.clone()).unwrap_or_default(),
                game.map(|g| BookmarkThumbnail {
                    kind: BookmarkThumbnailKind::GameIcon,
                    id: Id(g.universe_id),
                }),
                game.map(|g| format!("{} playing", fmt::count(g.playing)))
                    .unwrap_or_default(),
            )
        }
        "users" => {
            let name = s
                .users
                .get(&id)
                .map(|u| u.display_name.clone())
                .unwrap_or_default();
            let status = s
                .data
                .get(&account)
                .and_then(|a| a.presence.get(&id))
                .map(|p| {
                    if p.last_location.is_empty() {
                        p.kind.label().into()
                    } else {
                        p.last_location.clone()
                    }
                })
                .unwrap_or_default();
            (name, thumb(BookmarkThumbnailKind::Headshot, id), status)
        }
        "catalog" | "bundles" => {
            drop(s);
            let item = core.cached_bookmark_item(path);
            (
                item.as_ref().map(|i| i.name.clone()).unwrap_or_default(),
                thumb(
                    if route == "bundles" {
                        BookmarkThumbnailKind::Bundle
                    } else {
                        BookmarkThumbnailKind::Asset
                    },
                    id,
                ),
                item.as_ref().map(|i| i.price_label()).unwrap_or_default(),
            )
        }
        "passes" => (
            String::new(),
            thumb(BookmarkThumbnailKind::GamePass, id),
            String::new(),
        ),
        "communities" => {
            drop(s);
            let group = core.cached_bookmark_community(id);
            (
                group.as_ref().map(|g| g.name.clone()).unwrap_or_default(),
                thumb(BookmarkThumbnailKind::GroupIcon, id),
                group
                    .as_ref()
                    .map(|g| format!("{} members", fmt::count(g.members)))
                    .unwrap_or_default(),
            )
        }
        _ => (String::new(), None, String::new()),
    }
}

#[tauri::command]
#[specta::specta]
pub fn bookmarks(core: State<'_, Arc<Core>>) -> BookmarkList {
    let account = viewer(&core);
    let items = core
        .bookmarks(account)
        .into_iter()
        .map(|b| {
            let (_, thumbnail, status) = details(&core, account, &b.path);
            BookmarkView {
                id: Id(b.id),
                path: b.path,
                title: b.title,
                kind: b.kind.into(),
                thumbnail,
                status,
                folder: b.folder,
                tags: b.tags,
            }
        })
        .collect();
    BookmarkList {
        account: Id(account.0),
        items,
    }
}

#[derive(Deserialize)]
struct FolderRename {
    from: String,
    to: String,
}

/// Asks where to save, then writes the account's bookmarks. The dialog gets its own thread.
fn export(app: &AppHandle, account: AccountId) {
    let app = app.clone();
    std::thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new()
            .set_title("Save bookmarks")
            .set_file_name("bonsai-bookmarks.json")
            .add_filter("Bookmarks", &["json"]);
        if let Some(w) = app.get_webview_window("main") {
            dialog = dialog.set_parent(&w);
        }
        let Some(path) = dialog.save_file() else {
            return;
        };
        let Some(core) = app.try_state::<Arc<Core>>() else {
            return;
        };
        match core.export_bookmarks(account, &path) {
            Ok(n) => core.notice(format!("Saved {n} bookmarks to {}", path.display()), false),
            Err(e) => core.notice(e, true),
        }
    });
}

/// Asks for a bookmarks file, then adds what it holds. The dialog gets its own thread.
fn import(app: &AppHandle, account: AccountId) {
    let app = app.clone();
    std::thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new()
            .set_title("Load bookmarks")
            .add_filter("Bookmarks", &["json"]);
        if let Some(w) = app.get_webview_window("main") {
            dialog = dialog.set_parent(&w);
        }
        let Some(path) = dialog.pick_file() else {
            return;
        };
        let Some(core) = app.try_state::<Arc<Core>>() else {
            return;
        };
        match core.import_bookmarks(account, &path) {
            Ok(r) if r.skipped > 0 => core.notice(
                format!(
                    "Added {} bookmarks. Skipped {} already saved or not allowed.",
                    r.added, r.skipped
                ),
                false,
            ),
            Ok(r) => core.notice(format!("Added {} bookmarks.", r.added), false),
            Err(e) => core.notice(e, true),
        }
    });
}

#[derive(Deserialize)]
struct Target {
    path: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    fallback: String,
}

pub fn run(
    app: &AppHandle,
    core: &Arc<Core>,
    id: CommandId,
    inv: &Invocation,
    acting: Option<AccountId>,
) -> Option<Result<(), String>> {
    if !matches!(
        id,
        CommandId::ToggleBookmark
            | CommandId::OpenBookmark
            | CommandId::RenameBookmark
            | CommandId::RemoveBookmark
            | CommandId::MoveBookmarkUp
            | CommandId::MoveBookmarkDown
            | CommandId::MoveBookmark
            | CommandId::FolderBookmark
            | CommandId::RenameBookmarkFolder
            | CommandId::RemoveBookmarkFolder
            | CommandId::TagBookmark
            | CommandId::ExportBookmarks
            | CommandId::ImportBookmarks
    ) {
        return None;
    }
    Some((|| {
        let account = acting.unwrap_or(AccountId::ANONYMOUS);
        let raw = inv.arg.as_deref().unwrap_or("").trim();
        match id {
            CommandId::ExportBookmarks => {
                export(app, account);
                return Ok(());
            }
            CommandId::ImportBookmarks => {
                import(app, account);
                return Ok(());
            }
            CommandId::RenameBookmarkFolder => {
                let r: FolderRename =
                    serde_json::from_str(raw).map_err(|_| "Choose a folder and a new name.")?;
                return core
                    .edit_bookmark_folder(account, |all| bookmarks::rename_folder(all, &r.from, &r.to));
            }
            CommandId::RemoveBookmarkFolder => {
                return core.edit_bookmark_folder(account, |all| bookmarks::remove_folder(all, raw));
            }
            _ => {}
        }
        if id == CommandId::ToggleBookmark {
            let target: Target = serde_json::from_str(raw).map_err(|_| "Choose a page to bookmark.")?;
            let (cached, _, _) = details(core, account, &target.path);
            let title = if !target.title.trim().is_empty() {
                target.title
            } else if !cached.is_empty() {
                cached
            } else if !target.fallback.trim().is_empty() {
                target.fallback
            } else {
                target.path.clone()
            };
            core.edit_bookmarks(account, |all| {
                bookmarks::toggle(all, account, &target.path, &title)
            })?;
            return Ok(());
        }
        let (first, rest) = raw.split_once(char::is_whitespace).unwrap_or((raw, ""));
        let bookmark = first.parse().map_err(|_| "Choose a bookmark.")?;
        if id == CommandId::OpenBookmark {
            return core
                .bookmarks(account)
                .iter()
                .find(|b| b.id == bookmark)
                .map(|_| ())
                .ok_or_else(|| "No such bookmark.".into());
        }
        core.edit_bookmarks(account, |all| match id {
            CommandId::RenameBookmark => bookmarks::rename(all, bookmark, rest),
            CommandId::RemoveBookmark => bookmarks::remove(all, bookmark),
            CommandId::FolderBookmark => bookmarks::set_folder(all, bookmark, rest),
            CommandId::TagBookmark => bookmarks::set_tags(all, bookmark, rest),
            CommandId::MoveBookmarkUp => bookmarks::reorder(all, bookmark, -1, None),
            CommandId::MoveBookmarkDown => bookmarks::reorder(all, bookmark, 1, None),
            CommandId::MoveBookmark => bookmarks::reorder(
                all,
                bookmark,
                0,
                Some(rest.trim().parse().map_err(|_| "Choose a target bookmark.")?),
            ),
            _ => unreachable!("bookmark command checked above"),
        })
    })())
}
