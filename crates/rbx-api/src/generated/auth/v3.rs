//! `auth.roblox.com` v3 — generated from `vendor/roblox-api-tracker/auth/v3.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "auth.roblox.com";

/// `Roblox.Authentication.Api.Models.Request.SecureAuthenticationIntentModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SecureAuthenticationIntentModel {
    #[serde(rename = "clientEpochTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_epoch_timestamp: Option<i64>,
    #[serde(rename = "clientPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[serde(rename = "saiSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sai_signature: Option<String>,
    #[serde(rename = "serverNonce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_nonce: Option<String>,
}

/// `Roblox.Authentication.Api.Models.TwoStepVerificationV3LoginResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStepVerificationV3LoginResponse {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(rename = "identityVerificationLoginTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_verification_login_ticket: Option<String>,
}

/// `Roblox.Authentication.Api.TwoStepVerificationLoginRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStepVerificationLoginRequest {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    #[serde(rename = "rememberDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_device: Option<bool>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
    #[serde(rename = "verificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

/// `POST /v3/users/{userId}/two-step-verification/login`
///
/// Authenticates as a user given a two step verification verification token.
pub fn post_users_by_user_id_two_step_verification_login_request(
    user_id: i64,
    body: &TwoStepVerificationLoginRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v3/users/{}/two-step-verification/login",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v3/users/{userId}/two-step-verification/login`
///
/// Authenticates as a user given a two step verification verification token.
pub async fn post_users_by_user_id_two_step_verification_login(
    client: &Client,
    user_id: i64,
    body: &TwoStepVerificationLoginRequest,
) -> Result<TwoStepVerificationV3LoginResponse> {
    client
        .send(post_users_by_user_id_two_step_verification_login_request(
            user_id, body,
        )?)
        .await?
        .json()
}
