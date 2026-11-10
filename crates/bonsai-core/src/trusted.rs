//! Trusted friendships. Reads stay private to the viewing account; writes require a command.

use bonsai_api::generated::friends;
use bonsai_api::{AccountId, Error};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::effects::Effect;
use crate::people::FriendRequest;
use crate::res::{Key, Page, Res};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustedState {
    None,
    Trusted,
    Sent,
    Received,
    #[default]
    #[serde(other)]
    Unknown,
}

impl TrustedState {
    fn from_status(status: Option<&str>) -> Self {
        match status {
            Some("Friends" | "NotFriends" | "RequestIgnored") => Self::None,
            Some("TrustedFriends") => Self::Trusted,
            Some("RequestSent") => Self::Sent,
            Some("RequestReceived") => Self::Received,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Copy)]
enum Action {
    Send,
    Accept,
    Decline,
    Cancel,
    Remove,
}

impl Core {
    /// An independent resource: a trusted API failure must not hide ordinary friend actions.
    pub fn trusted_friend(&self, account: AccountId, user_id: u64) -> Res<TrustedState> {
        if account.is_anonymous() || account.0 == user_id {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "trusted_status", user_id),
            60,
            move |f| async move {
                let r = friends::undocumented::get_trusted_friend_status(&f.client, user_id as i64).await?;
                Ok(TrustedState::from_status(r.status.as_deref()))
            },
        )
    }

    pub fn trusted_requests(&self, account: AccountId, more: bool) -> Res<Page<FriendRequest>> {
        self.inner.paged(
            Key::private(account, "trusted_requests", ""),
            60,
            more,
            move |f, cursor| async move {
                let p = friends::v1::GetMyTrustedFriendsRequestsParams {
                    limit: Some(50),
                    cursor,
                    sort_order: Some(1),
                };
                let r = friends::v1::get_my_trusted_friends_requests(&f.client, &p).await?;
                let items = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|r| {
                        let user_id = u64::try_from(r.sender_id?).ok().filter(|id| *id > 0)?;
                        Some(FriendRequest {
                            user_id,
                            sent_at: r.sent_at.unwrap_or_default(),
                            ..Default::default()
                        })
                    })
                    .collect();
                Ok(Page {
                    items,
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    pub fn add_trusted_friend(&self, account: AccountId, user_id: u64) {
        self.change_trusted_friend(account, user_id, Action::Send);
    }

    pub fn accept_trusted_friend(&self, account: AccountId, user_id: u64) {
        self.change_trusted_friend(account, user_id, Action::Accept);
    }

    pub fn decline_trusted_friend(&self, account: AccountId, user_id: u64) {
        self.change_trusted_friend(account, user_id, Action::Decline);
    }

    pub fn cancel_trusted_friend(&self, account: AccountId, user_id: u64) {
        self.change_trusted_friend(account, user_id, Action::Cancel);
    }

    pub fn remove_trusted_friend(&self, account: AccountId, user_id: u64) {
        self.change_trusted_friend(account, user_id, Action::Remove);
    }

    fn change_trusted_friend(&self, account: AccountId, user_id: u64, action: Action) {
        let (what, done) = match action {
            Action::Send => ("send the trusted friend request", "Trusted friend request sent"),
            Action::Accept => (
                "accept the trusted friend request",
                "Trusted friend request accepted",
            ),
            Action::Decline => (
                "decline the trusted friend request",
                "Trusted friend request declined",
            ),
            Action::Cancel => (
                "cancel the trusted friend request",
                "Trusted friend request canceled",
            ),
            Action::Remove => (
                "remove the trusted friend",
                "Trusted status removed. You are still friends.",
            ),
        };
        self.inner.mutate(
            account,
            what,
            done,
            &[Effect::TrustedFriendship(user_id)],
            move |f| async move {
                if user_id == 0 || user_id == account.0 || user_id > i64::MAX as u64 {
                    return Err(Error::Message("Choose another user.".into()));
                }
                let body = friends::undocumented::Empty::default();
                match action {
                    Action::Send => {
                        friends::undocumented::post_send_trusted_friend_request(
                            &f.client,
                            user_id as i64,
                            &body,
                        )
                        .await?
                    }
                    Action::Accept => {
                        friends::undocumented::post_accept_trusted_friend_request(
                            &f.client,
                            user_id as i64,
                            &body,
                        )
                        .await?
                    }
                    Action::Decline => {
                        friends::undocumented::post_ignore_trusted_friend_request(
                            &f.client,
                            user_id as i64,
                            &body,
                        )
                        .await?
                    }
                    Action::Cancel | Action::Remove => {
                        friends::undocumented::post_remove_trusted_friend(&f.client, user_id as i64, &body)
                            .await?
                    }
                };
                Ok(())
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_status_never_offers_to_send_a_request() {
        assert_eq!(TrustedState::from_status(None), TrustedState::Unknown);
        assert_eq!(
            TrustedState::from_status(Some("FutureStatus")),
            TrustedState::Unknown
        );
        assert_eq!(
            TrustedState::from_status(Some("TrustedFriends")),
            TrustedState::Trusted
        );
        assert_eq!(
            TrustedState::from_status(Some("RequestReceived")),
            TrustedState::Received
        );
        assert_eq!(TrustedState::from_status(Some("RequestSent")), TrustedState::Sent);
        assert_eq!(TrustedState::from_status(Some("Friends")), TrustedState::None);
        assert_eq!(
            TrustedState::from_status(Some("RequestIgnored")),
            TrustedState::None
        );
        assert_eq!(TrustedState::from_status(Some("Invalid")), TrustedState::Unknown);
    }
}
