//! What a change on Roblox makes out of date.
//!
//! Writes ([`Inner::mutate`]) and watchers (realtime events, the health loop, the friends and
//! sales watchers) say what happened as an [`Effect`]. [`Inner::apply`] is the one table that
//! knows which cached resources each effect touches, so a click and a watcher that see the
//! same change refresh the same things. Applying an effect only marks data stale; the next read
//! refetches it while still showing the old copy.

use bonsai_api::AccountId;

use crate::core::Inner;
use crate::res::Key;

/// Something that changed for an account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// The account's Robux balance moved (a purchase, a sale, a trade with Robux).
    Robux,
    /// What the account owns changed: bought, sold, traded or deleted items and passes.
    Inventory,
    /// Resale listings and item details, after buying from them.
    Market,
    /// The account's trades: lists and details.
    Trades,
    /// The account's friendship with this user started or ended.
    Friendship(u64),
    /// Friend requests to the account.
    FriendRequests,
    /// The account followed or unfollowed this user.
    Follow(u64),
    /// The account blocked or unblocked this user.
    Block(u64),
    /// This user's own profile details (name, display name, description).
    Profile(u64),
    /// What the avatar is wearing and how it looks.
    Avatar,
    /// Saved outfits.
    Outfits,
    /// Chat conversations and their messages.
    Chat,
    /// Private messages (the inbox).
    Messages,
    /// The notification stream.
    Notifications,
    /// The account joined, left or changed its primary community.
    Membership,
    /// A community's details, members, roles, requests, settings, affiliates or audit log.
    Community,
    /// Community forum posts and comments.
    Forum,
    /// Community funds and payouts.
    CommunityFunds,
    /// The account's vote, favorite or follow on an experience.
    GameState,
    /// Experience events (RSVPs).
    GameEvents,
    /// Private servers and their settings.
    PrivateServers,
    /// The account's favorite on a catalog item.
    ItemFavorite,
    /// The account PIN lock.
    Pin,
    /// Privacy settings.
    Privacy,
    /// Password, 2-step and session settings.
    Security,
}

/// Resource kinds to mark stale for every key of the account (and the shared copy).
fn kinds(e: Effect) -> &'static [&'static str] {
    match e {
        Effect::Robux => &["robux", "tx_totals"],
        Effect::Inventory => &[
            "owns",
            "collectibles",
            "tradable",
            "inventory",
            "game_passes",
            "user_passes",
        ],
        Effect::Market => &["item", "items", "resellers"],
        Effect::Trades => &["trades", "trade", "tradable"],
        Effect::Friendship(_) => &["friends_search"],
        Effect::FriendRequests => &["friend_requests", "relation"],
        Effect::Follow(_) | Effect::Block(_) => &[],
        Effect::Profile(_) => &[],
        Effect::Avatar => &["avatar"],
        Effect::Outfits => &["outfits"],
        Effect::Chat => &["chat_conversations", "chat_messages"],
        Effect::Messages => &["private_messages"],
        Effect::Notifications => &["notifications"],
        Effect::Membership => &[
            "community_membership",
            "user_groups",
            "community_pending",
            "community",
        ],
        Effect::Community => &[
            "community",
            "community_members",
            "community_roles",
            "community_requests",
            "community_settings",
            "community_affiliates",
            "community_audit",
        ],
        Effect::Forum => &["forum_posts", "forum_comments", "community_audit"],
        Effect::CommunityFunds => &["community_funds", "community_payouts", "community_audit"],
        Effect::GameState => &["my_game_state"],
        Effect::GameEvents => &["game_events"],
        Effect::PrivateServers => &["private_servers", "my_private_servers", "server_config"],
        Effect::ItemFavorite => &["item_fav"],
        Effect::Pin => &["pin"],
        Effect::Privacy => &["privacy"],
        Effect::Security => &["security", "sessions"],
    }
}

/// Single resources to mark stale, beyond whole kinds.
fn keys(account: AccountId, e: Effect) -> Vec<Key> {
    match e {
        Effect::Friendship(id) => vec![
            Key::private(account, "friends_of", id),
            Key::shared("profile_counts", id),
            Key::private(account, "relation", id),
        ],
        Effect::Follow(id) => vec![
            Key::shared("profile_counts", id),
            Key::private(account, "relation", id),
            Key::private(account, "followers", id),
            Key::private(account, "followings", account.0),
        ],
        Effect::Block(id) => vec![
            Key::private(account, "relation", id),
            Key::private(account, "blocked", ""),
        ],
        Effect::Profile(id) => vec![Key::shared("profile", id)],
        _ => Vec::new(),
    }
}

impl Inner {
    /// Mark what `effects` made out of date for `account`, then repaint so open screens refetch.
    pub(crate) fn apply(&self, account: AccountId, effects: &[Effect]) {
        for e in effects {
            for kind in kinds(*e) {
                self.expire_kind(account, kind);
                self.expire_kind(AccountId::ANONYMOUS, kind);
            }
            for k in keys(account, *e) {
                self.expire(&k);
            }
        }
        if !effects.is_empty() {
            self.repaint();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendship_touches_both_sides() {
        let a = AccountId(1);
        let k = keys(a, Effect::Friendship(7));
        assert!(k.contains(&Key::private(a, "friends_of", 7)));
        assert!(k.contains(&Key::private(a, "relation", 7)));
    }

    #[test]
    fn robux_covers_the_balance() {
        assert!(kinds(Effect::Robux).contains(&"robux"));
    }
}
