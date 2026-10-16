//! `voice.roblox.com` undocumented — generated from `specs/undocumented/voice.roblox.com.yaml`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "voice.roblox.com";

/// `VoiceSettings`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VoiceSettings {
    #[serde(rename = "isAvatarVideoEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avatar_video_eligible: Option<bool>,
    #[serde(rename = "isAvatarVideoEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avatar_video_enabled: Option<bool>,
    #[serde(rename = "isBanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<bool>,
    #[serde(rename = "isUserEligible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_eligible: Option<bool>,
    #[serde(rename = "isVerifiedForVoice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified_for_voice: Option<bool>,
    #[serde(rename = "isVoiceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_voice_enabled: Option<bool>,
}

/// `GET /v1/settings`
///
/// Voice chat and avatar camera status of the account.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/privacy.
pub fn get_voice_settings_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/settings".to_string());
    Ok(r)
}

/// `GET /v1/settings`
///
/// Voice chat and avatar camera status of the account.
///
/// Undocumented; captured 2026-10-16, www.roblox.com/my/account#!/privacy.
pub async fn get_voice_settings(client: &Client) -> Result<VoiceSettings> {
    client.send(get_voice_settings_request()?).await?.json()
}
