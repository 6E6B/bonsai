//! `groups.roblox.com` undocumented — generated from `specs/undocumented/groups.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "groups.roblox.com";

/// `Empty`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Empty {}

/// `ForumCategory`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isArchived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "isRestricted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(rename = "shortId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
}

/// `ForumCategories`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumCategories {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ForumCategory>>,
}

/// `ForumContent`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumContent {
    #[serde(rename = "plainText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
}

/// `ForumCreator`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumCreator {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "groupRoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_role_name: Option<String>,
    #[serde(rename = "hasVerifiedBadge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_verified_badge: Option<bool>,
}

/// `ForumComment`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumComment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ForumContent>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i64>,
    #[serde(rename = "creatorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_info: Option<ForumCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

/// `ForumCommentsPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumCommentsPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ForumComment>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// `ForumNewComment`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumNewComment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "repliesToPostCommentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies_to_post_comment_id: Option<String>,
}

/// `ForumNewPost`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumNewPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// `ForumPost`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumPost {
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(rename = "commentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i32>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i64>,
    #[serde(rename = "firstComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_comment: Option<ForumComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isLocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(rename = "isPinned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_pinned: Option<bool>,
    #[serde(rename = "isUnread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unread: Option<bool>,
    /// The post's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// `ForumPostsPage`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForumPostsPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ForumPost>>,
    #[serde(rename = "nextPageCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
}

/// Query, header and form inputs for [`get_forum_categories`].
#[derive(Debug, Clone, Default)]
pub struct GetForumCategoriesParams {
    pub archived: Option<bool>,
}

/// Query, header and form inputs for [`get_forum_posts`].
#[derive(Debug, Clone, Default)]
pub struct GetForumPostsParams {
    pub include_comment_count: Option<bool>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// Query, header and form inputs for [`get_forum_pinned_posts`].
#[derive(Debug, Clone, Default)]
pub struct GetForumPinnedPostsParams {
    pub include_comment_count: Option<bool>,
}

/// Query, header and form inputs for [`get_forum_comments`].
#[derive(Debug, Clone, Default)]
pub struct GetForumCommentsParams {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

/// `GET /v1/groups/{groupId}/forums`
///
/// A community's forum categories (archived ones with archived=true).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn get_forum_categories_request(group_id: i64, params: &GetForumCategoriesParams) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/groups/{}/forums", crate::rt::path_value(&group_id)),
    );
    r.query("archived", &params.archived);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/forums`
///
/// A community's forum categories (archived ones with archived=true).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn get_forum_categories(
    client: &Client,
    group_id: i64,
    params: &GetForumCategoriesParams,
) -> Result<ForumCategories> {
    client
        .send(get_forum_categories_request(group_id, params)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/forums/{categoryId}/posts`
///
/// Posts in a forum category, newest activity first.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn get_forum_posts_request(
    group_id: i64,
    category_id: &str,
    params: &GetForumPostsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id)
        ),
    );
    r.query("includeCommentCount", &params.include_comment_count);
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/forums/{categoryId}/posts`
///
/// Posts in a forum category, newest activity first.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn get_forum_posts(
    client: &Client,
    group_id: i64,
    category_id: &str,
    params: &GetForumPostsParams,
) -> Result<ForumPostsPage> {
    client
        .send(get_forum_posts_request(group_id, category_id, params)?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/forums/{categoryId}/posts`
///
/// Start a post in a category (user action only).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn post_forum_post_request(group_id: i64, category_id: &str, body: &ForumNewPost) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/forums/{categoryId}/posts`
///
/// Start a post in a category (user action only).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn post_forum_post(
    client: &Client,
    group_id: i64,
    category_id: &str,
    body: &ForumNewPost,
) -> Result<ForumPost> {
    client
        .send(post_forum_post_request(group_id, category_id, body)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/forums/{categoryId}/posts/pinned`
///
/// Pinned posts of a category.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn get_forum_pinned_posts_request(
    group_id: i64,
    category_id: &str,
    params: &GetForumPinnedPostsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts/pinned",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id)
        ),
    );
    r.query("includeCommentCount", &params.include_comment_count);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/forums/{categoryId}/posts/pinned`
///
/// Pinned posts of a category.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn get_forum_pinned_posts(
    client: &Client,
    group_id: i64,
    category_id: &str,
    params: &GetForumPinnedPostsParams,
) -> Result<ForumPostsPage> {
    client
        .send(get_forum_pinned_posts_request(group_id, category_id, params)?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}`
///
/// Delete a post (its author or a moderator).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn delete_forum_post_request(group_id: i64, category_id: &str, post_id: &str) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id),
            crate::rt::path_value(&post_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}`
///
/// Delete a post (its author or a moderator).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn delete_forum_post(
    client: &Client,
    group_id: i64,
    category_id: &str,
    post_id: &str,
) -> Result<Empty> {
    client
        .send(delete_forum_post_request(group_id, category_id, post_id)?)
        .await?
        .json()
}

/// `GET /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}/comments`
///
/// Comments on a post, oldest first. The first one is the post's own text.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn get_forum_comments_request(
    group_id: i64,
    category_id: &str,
    post_id: &str,
    params: &GetForumCommentsParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts/{}/comments",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id),
            crate::rt::path_value(&post_id)
        ),
    );
    r.query("limit", &params.limit);
    r.query("cursor", &params.cursor);
    Ok(r)
}

/// `GET /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}/comments`
///
/// Comments on a post, oldest first. The first one is the post's own text.
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn get_forum_comments(
    client: &Client,
    group_id: i64,
    category_id: &str,
    post_id: &str,
    params: &GetForumCommentsParams,
) -> Result<ForumCommentsPage> {
    client
        .send(get_forum_comments_request(
            group_id,
            category_id,
            post_id,
            params,
        )?)
        .await?
        .json()
}

/// `POST /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}/comments`
///
/// Reply to a post (user action only).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn post_forum_comment_request(
    group_id: i64,
    category_id: &str,
    post_id: &str,
    body: &ForumNewComment,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts/{}/comments",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id),
            crate::rt::path_value(&post_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}/comments`
///
/// Reply to a post (user action only).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn post_forum_comment(
    client: &Client,
    group_id: i64,
    category_id: &str,
    post_id: &str,
    body: &ForumNewComment,
) -> Result<ForumComment> {
    client
        .send(post_forum_comment_request(group_id, category_id, post_id, body)?)
        .await?
        .json()
}

/// `DELETE /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}/comments/{commentId}`
///
/// Delete a comment (its author or a moderator).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub fn delete_forum_comment_request(
    group_id: i64,
    category_id: &str,
    post_id: &str,
    comment_id: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::DELETE,
        HOST,
        format!(
            "/v1/groups/{}/forums/{}/posts/{}/comments/{}",
            crate::rt::path_value(&group_id),
            crate::rt::path_value(&category_id),
            crate::rt::path_value(&post_id),
            crate::rt::path_value(&comment_id)
        ),
    );
    Ok(r)
}

/// `DELETE /v1/groups/{groupId}/forums/{categoryId}/posts/{postId}/comments/{commentId}`
///
/// Delete a comment (its author or a moderator).
///
/// Undocumented; captured 2026-10-13, www.roblox.com/communities/{id}#!/forums.
pub async fn delete_forum_comment(
    client: &Client,
    group_id: i64,
    category_id: &str,
    post_id: &str,
    comment_id: &str,
) -> Result<Empty> {
    client
        .send(delete_forum_comment_request(
            group_id,
            category_id,
            post_id,
            comment_id,
        )?)
        .await?
        .json()
}
