//! Trusted friendships use the same authenticated, CSRF-aware transport as other social writes.
#![allow(clippy::unwrap_used)]

use bonsai_api::generated::friends::{undocumented as u, v1};
use bonsai_api::replay::Replay;

#[tokio::test]
async fn trusted_statuses_requests_paging_and_actions() {
    let replay = Replay::start(&["trusted_friends"]).await;
    let c = replay.client();
    for (id, status) in [
        (2000001, "Friends"),
        (2000002, "RequestReceived"),
        (2000003, "RequestSent"),
        (2000004, "TrustedFriends"),
    ] {
        assert_eq!(
            u::get_trusted_friend_status(&c, id)
                .await
                .unwrap()
                .status
                .as_deref(),
            Some(status)
        );
    }
    let mut params = v1::GetMyTrustedFriendsRequestsParams {
        limit: Some(50),
        sort_order: Some(1),
        ..Default::default()
    };
    let first = v1::get_my_trusted_friends_requests(&c, &params).await.unwrap();
    assert_eq!(first.data.unwrap()[0].sender_id, Some(2000002));
    params.cursor = first.next_page_cursor;
    let second = v1::get_my_trusted_friends_requests(&c, &params).await.unwrap();
    assert_eq!(second.data.unwrap()[0].sender_id, Some(2000005));
    assert!(second.next_page_cursor.is_none());
    let body = u::Empty::default();
    u::post_send_trusted_friend_request(&c, 2000001, &body)
        .await
        .unwrap();
    u::post_accept_trusted_friend_request(&c, 2000002, &body)
        .await
        .unwrap();
    u::post_ignore_trusted_friend_request(&c, 2000005, &body)
        .await
        .unwrap();
    u::post_remove_trusted_friend(&c, 2000003, &body).await.unwrap();
    u::post_remove_trusted_friend(&c, 2000004, &body).await.unwrap();
    let requests = replay.requests().await;
    assert_eq!(requests.len(), 11);
    for request in requests.iter().filter(|r| r.method == "POST") {
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&request.body).unwrap(),
            serde_json::json!({})
        );
        assert!(request.headers.contains_key("cookie"));
    }
}

#[tokio::test]
async fn refused_trusted_request_is_an_error_and_is_not_retried() {
    let mut fixture = bonsai_api::replay::load("trusted_friends");
    fixture
        .interactions
        .retain(|i| i.request.path.ends_with("/send-trusted-friend-request"));
    fixture.interactions[0].response.status = 400;
    fixture.interactions[0].response.body = serde_json::json!({"errors":[{"code":1,"message":"This account is not eligible for trusted friends."}]});
    let replay = Replay::from_fixture(&fixture).await;
    let error = u::post_send_trusted_friend_request(&replay.client(), 2000001, &u::Empty::default())
        .await
        .unwrap_err();
    assert!(error.user_message().contains("not eligible"));
    assert_eq!(replay.requests().await.len(), 1);
}
