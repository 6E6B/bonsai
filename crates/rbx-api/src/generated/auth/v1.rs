//! `auth.roblox.com` v1 — generated from `vendor/roblox-api-tracker/auth/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "auth.roblox.com";

/// `Roblox.Authentication.Api.Models.AccountPinRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountPinRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    #[serde(rename = "reauthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reauthentication_token: Option<String>,
}

/// `Roblox.Authentication.Api.Models.AccountPinResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountPinResponse {
    #[serde(rename = "unlockedUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<f64>,
}

/// `Roblox.Authentication.Api.Models.AccountPinStatusResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountPinStatusResponse {
    #[serde(rename = "isEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "unlockedUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocked_until: Option<f64>,
}

/// `Roblox.Authentication.Api.Models.AuthMetaDataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AuthMetaDataResponse {
    #[serde(rename = "cookieLawNoticeTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_law_notice_timeout: Option<i32>,
}

/// `Roblox.Authentication.Api.Models.EmailValidationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmailValidationResponse {
    #[serde(rename = "isEmailValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_email_valid: Option<bool>,
}

crate::api_enum! {
    int
    /// \['EmailOtp' = 0, 'Passkey' = 1, 'Password' = 2\]
    LoginMethodModelMethod {
    EmailOtp = 0 => "EmailOtp",
    Passkey = 1 => "Passkey",
    Password = 2 => "Password",
    }
}

/// `Roblox.Authentication.Api.Models.LoginMethodModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LoginMethodModel {
    /// \['EmailOtp' = 0, 'Passkey' = 1, 'Password' = 2\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<LoginMethodModelMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

crate::api_enum! {
    int
    /// \['Email' = 0, 'Username' = 1, 'PhoneNumber' = 2, 'EmailOtpSessionToken' = 3, 'AuthToken' = 4, 'Passkey' = 5\]
    LoginRequestCtype {
    Email = 0 => "Email",
    Username = 1 => "Username",
    PhoneNumber = 2 => "PhoneNumber",
    EmailOtpSessionToken = 3 => "EmailOtpSessionToken",
    AuthToken = 4 => "AuthToken",
    Passkey = 5 => "Passkey",
    }
}

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

/// `Roblox.Authentication.Api.Models.LoginRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LoginRequest {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(rename = "captchaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,
    #[serde(rename = "captchaProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_provider: Option<String>,
    #[serde(rename = "captchaToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    /// \['Email' = 0, 'Username' = 1, 'PhoneNumber' = 2, 'EmailOtpSessionToken' = 3, 'AuthToken' = 4, 'Passkey' = 5\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<LoginRequestCtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvalue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
    #[serde(rename = "securityQuestionRedemptionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_question_redemption_token: Option<String>,
    #[serde(rename = "securityQuestionSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_question_session_id: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

crate::api_enum! {
    int
    /// \['Email' = 0, 'SMS' = 1, 'Authenticator' = 2, 'RecoveryCode' = 3, 'SecurityKey' = 4, 'CrossDevice' = 5, 'Password' = 6, 'Passkey' = 7\]
    TwoStepVerificationSentResponseMediaType {
    Email = 0 => "Email",
    Sms = 1 => "SMS",
    Authenticator = 2 => "Authenticator",
    RecoveryCode = 3 => "RecoveryCode",
    SecurityKey = 4 => "SecurityKey",
    CrossDevice = 5 => "CrossDevice",
    Password = 6 => "Password",
    Passkey = 7 => "Passkey",
    }
}

/// `Roblox.Authentication.Api.Models.TwoStepVerificationSentResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStepVerificationSentResponse {
    /// \['Email' = 0, 'SMS' = 1, 'Authenticator' = 2, 'RecoveryCode' = 3, 'SecurityKey' = 4, 'CrossDevice' = 5, 'Password' = 6, 'Passkey' = 7\]
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<TwoStepVerificationSentResponseMediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}

/// `Roblox.Web.Responses.Users.SkinnyUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SkinnyUserResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Authentication.Api.Models.LoginResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LoginResponse {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(rename = "identityVerificationLoginTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_verification_login_ticket: Option<String>,
    #[serde(rename = "isBanned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_banned: Option<bool>,
    #[serde(rename = "twoStepVerificationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_verification_data: Option<TwoStepVerificationSentResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SkinnyUserResponse>,
}

/// `Roblox.Authentication.Api.Models.MetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MetadataResponse {
    #[serde(rename = "IsAccountRecoveryPromptEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_recovery_prompt_enabled: Option<bool>,
    #[serde(rename = "IsAltBrowserTracker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_alt_browser_tracker: Option<bool>,
    #[serde(rename = "IsContactMethodRequiredAtSignup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contact_method_required_at_signup: Option<bool>,
    #[serde(rename = "IsEmailUpsellAtLogoutEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_email_upsell_at_logout_enabled: Option<bool>,
    #[serde(rename = "IsKoreaIdVerificationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_korea_id_verification_enabled: Option<bool>,
    #[serde(rename = "IsPasskeyFeatureEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_passkey_feature_enabled: Option<bool>,
    #[serde(rename = "IsPasswordRequiredForUsernameChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_username_change: Option<bool>,
    #[serde(rename = "IsUserAgreementsSignupIntegrationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_agreements_signup_integration_enabled: Option<bool>,
    #[serde(rename = "ShouldFetchEmailUpsellIXPValuesAtLogout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_fetch_email_upsell_ixp_values_at_logout: Option<bool>,
    #[serde(rename = "ftuxAvatarAssetMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftux_avatar_asset_map: Option<String>,
    #[serde(rename = "isUpdateUsernameEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_update_username_enabled: Option<bool>,
}

/// `Roblox.Authentication.Api.Models.PasswordChangeModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordChangeModel {
    #[serde(rename = "currentPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    #[serde(rename = "newPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
}

/// `Roblox.Authentication.Api.Models.PasswordValidationModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordValidationModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

crate::api_enum! {
    int
    /// \['ValidPassword' = 0, 'WeakPasswordError' = 1, 'ShortPasswordError' = 2, 'PasswordSameAsUsernameError' = 3, 'ForbiddenPasswordError' = 4, 'DumbStringsError' = 5\]
    PasswordValidationResponseCode {
    ValidPassword = 0 => "ValidPassword",
    WeakPasswordError = 1 => "WeakPasswordError",
    ShortPasswordError = 2 => "ShortPasswordError",
    PasswordSameAsUsernameError = 3 => "PasswordSameAsUsernameError",
    ForbiddenPasswordError = 4 => "ForbiddenPasswordError",
    DumbStringsError = 5 => "DumbStringsError",
    }
}

/// `Roblox.Authentication.Api.Models.PasswordValidationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordValidationResponse {
    /// \['ValidPassword' = 0, 'WeakPasswordError' = 1, 'ShortPasswordError' = 2, 'PasswordSameAsUsernameError' = 3, 'ForbiddenPasswordError' = 4, 'DumbStringsError' = 5\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<PasswordValidationResponseCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `Roblox.Authentication.Api.Models.ProviderInfoModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProviderInfoModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

/// `Roblox.Authentication.Api.Models.RecommendedUsernameFromDisplayNameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecommendedUsernameFromDisplayNameRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// `Roblox.Authentication.Api.Models.RecommendedUsernameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecommendedUsernameRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.RecommendedUsernameResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecommendedUsernameResponse {
    #[serde(rename = "didGenerateNewUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did_generate_new_username: Option<bool>,
    #[serde(rename = "suggestedUsernames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_usernames: Option<Vec<String>>,
}

crate::api_enum! {
    int
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    RecoverUsernameRequestTargetType {
    Email = 0 => "Email",
    PhoneNumber = 1 => "PhoneNumber",
    }
}

/// `Roblox.Authentication.Api.Models.RecoverUsernameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecoverUsernameRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<RecoverUsernameRequestTargetType>,
}

/// `Roblox.Authentication.Api.Models.RecoverUsernameResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecoverUsernameResponse {
    #[serde(rename = "transmissionType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_type: Option<i32>,
}

/// `Roblox.Authentication.Api.Models.RecoveryMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecoveryMetadataResponse {
    #[serde(rename = "codeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_length: Option<i32>,
    #[serde(rename = "isBedev2CaptchaEnabledForPasswordReset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bedev2_captcha_enabled_for_password_reset: Option<bool>,
    #[serde(rename = "isOnPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_phone: Option<bool>,
    #[serde(rename = "isPhoneFeatureEnabledForPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_phone_feature_enabled_for_password: Option<bool>,
    #[serde(rename = "isPhoneFeatureEnabledForUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_phone_feature_enabled_for_username: Option<bool>,
    #[serde(rename = "isUsernameRecoveryDeprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_username_recovery_deprecated: Option<bool>,
}

/// `Roblox.Authentication.Api.Models.ReferralDataModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ReferralDataModel {
    #[serde(rename = "acquisitionReferrer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_referrer: Option<String>,
    #[serde(rename = "acquisitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_time: Option<String>,
    #[serde(rename = "adGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename = "matchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(rename = "requestSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_session_id: Option<String>,
    #[serde(rename = "sendInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Request.DeletePasskeysRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeletePasskeysRequest {
    #[serde(rename = "credentialNicknames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_nicknames: Option<Vec<String>>,
}

crate::api_enum! {
    int
    /// \['Undefined' = 0, 'Xbox' = 1, 'Playstation' = 2, 'Web' = 3\]
    ExternalAccessRequestIdentityProviderPlatformType {
    Undefined = 0 => "Undefined",
    Xbox = 1 => "Xbox",
    Playstation = 2 => "Playstation",
    Web = 3 => "Web",
    }
}

/// `Roblox.Authentication.Api.Models.Request.ExternalAccessRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalAccessRequest {
    #[serde(rename = "additionalInfoPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info_payload: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "authenticationProof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_proof: Option<String>,
    /// \['Undefined' = 0, 'Xbox' = 1, 'Playstation' = 2, 'Web' = 3\]
    #[serde(rename = "identityProviderPlatformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_platform_type: Option<ExternalAccessRequestIdentityProviderPlatformType>,
}

/// `Roblox.Authentication.Api.Models.Request.ExternalLoginAndLinkRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalLoginAndLinkRequest {
    /// \['Undefined' = 0, 'Xbox' = 1, 'Playstation' = 2, 'Web' = 3\]
    #[serde(rename = "IdentityProviderPlatformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_platform_type: Option<ExternalAccessRequestIdentityProviderPlatformType>,
    #[serde(rename = "additionalInfoPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info_payload: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "authenticationProof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_proof: Option<String>,
    /// \['Email' = 0, 'Username' = 1, 'PhoneNumber' = 2, 'EmailOtpSessionToken' = 3, 'AuthToken' = 4, 'Passkey' = 5\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctype: Option<LoginRequestCtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvalue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Request.ExternalLoginRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalLoginRequest {
    #[serde(rename = "additionalInfoPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info_payload: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "authenticationProof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_proof: Option<String>,
    /// \['Undefined' = 0, 'Xbox' = 1, 'Playstation' = 2, 'Web' = 3\]
    #[serde(rename = "identityProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<ExternalAccessRequestIdentityProviderPlatformType>,
}

/// `Roblox.Authentication.Api.Models.Request.ExternalSignupRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalSignupRequest {
    /// \['Undefined' = 0, 'Xbox' = 1, 'Playstation' = 2, 'Web' = 3\]
    #[serde(rename = "IdentityProviderPlatformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_platform_type: Option<ExternalAccessRequestIdentityProviderPlatformType>,
    #[serde(rename = "additionalInfoPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info_payload: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "authenticationProof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_proof: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Request.ExternalUnlinkRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalUnlinkRequest {
    /// \['Undefined' = 0, 'Xbox' = 1, 'Playstation' = 2, 'Web' = 3\]
    #[serde(rename = "IdentityProviderPlatformType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_platform_type: Option<ExternalAccessRequestIdentityProviderPlatformType>,
    #[serde(rename = "additionalInfoPayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info_payload: Option<HashMap<String, serde_json::Value>>,
}

/// `Roblox.Authentication.Api.Models.Request.FinishPasskeyRegistrationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FinishPasskeyRegistrationRequest {
    #[serde(rename = "attestationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_response: Option<String>,
    #[serde(rename = "credentialNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_nickname: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Request.IdentityVerificationLoginRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct IdentityVerificationLoginRequest {
    #[serde(rename = "loginTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_ticket: Option<String>,
    #[serde(rename = "resultToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_token: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Request.InitializeLoginRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InitializeLoginRequest {
    #[serde(rename = "captchaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,
    #[serde(rename = "captchaProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_provider: Option<String>,
    #[serde(rename = "captchaToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    pub ctype: i32,
    pub cvalue: String,
}

/// `Roblox.Authentication.Api.Models.Request.ListPasskeysRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ListPasskeysRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

/// `Roblox.Authentication.Api.Models.Request.LogoutFromAllSessionsAndReauthenticateRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LogoutFromAllSessionsAndReauthenticateRequest {
    #[serde(rename = "SecureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
}

crate::api_enum! {
    int
    /// \['Unset' = 1, 'Email' = 2\]
    OtpSessionModelOtpContactType {
    Unset = 1 => "Unset",
    Email = 2 => "Email",
    }
}

/// `Roblox.Authentication.Api.Models.Request.OtpSessionModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OtpSessionModel {
    /// \['Unset' = 1, 'Email' = 2\]
    #[serde(rename = "otpContactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_contact_type: Option<OtpSessionModelOtpContactType>,
    #[serde(rename = "otpSessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_session_token: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Request.StartAuthenticationByUserRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartAuthenticationByUserRequest {
    pub ctype: i32,
    pub cvalue: String,
}

/// `Roblox.Authentication.Api.Models.Response.ExternalIdentityGateway.ExternalIdentityAccessResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ExternalIdentityAccessResponse {
    #[serde(rename = "isolationContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation_context: Option<String>,
    #[serde(rename = "launchData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_data: Option<String>,
    #[serde(rename = "placeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<i64>,
}

/// `Roblox.Authentication.Api.Models.Response.GetClientAssertionResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetClientAssertionResponse {
    #[serde(rename = "clientAssertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_assertion: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Response.InitializeLoginResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InitializeLoginResponse {
    #[serde(rename = "loginMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_methods: Option<Vec<LoginMethodModel>>,
}

/// `Roblox.Authentication.Api.Models.Response.PasskeyCredential`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasskeyCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Response.ListPasskeyCredentialResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ListPasskeyCredentialResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<PasskeyCredential>>,
}

/// `Roblox.Authentication.Api.Models.Response.StartAuthenticationByUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartAuthenticationByUserResponse {
    #[serde(rename = "authenticationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_options: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Response.StartAuthenticationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartAuthenticationResponse {
    #[serde(rename = "authenticationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_options: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.Authentication.Api.Models.Response.StartPasskeyRegistrationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct StartPasskeyRegistrationResponse {
    #[serde(rename = "creationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_options: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.Authentication.Api.Models.RevertAccountInfoResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RevertAccountInfoResponse {
    #[serde(rename = "isEmailChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_email_changed: Option<bool>,
    #[serde(rename = "isEmailVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_email_verified: Option<bool>,
    #[serde(rename = "isPhoneVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_phone_verified: Option<bool>,
    #[serde(rename = "isTwoStepVerificationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_two_step_verification_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.RevertAccountSubmitRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RevertAccountSubmitRequest {
    #[serde(rename = "NewPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(rename = "NewPasswordRepeated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password_repeated: Option<String>,
    #[serde(rename = "Ticket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(rename = "TwoStepVerificationChallengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_verification_challenge_id: Option<String>,
    #[serde(rename = "TwoStepVerificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_verification_token: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

crate::api_enum! {
    int
    /// \['Unknown' = 1, 'Male' = 2, 'Female' = 3\]
    SignupRequestGender {
    UnknownValue = 1 => "Unknown",
    Male = 2 => "Male",
    Female = 3 => "Female",
    }
}

/// `Roblox.Authentication.Api.Models.SignupRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SignupRequest {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(rename = "agreementIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_ids: Option<Vec<String>>,
    #[serde(rename = "assetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "bodyColorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_color_id: Option<i32>,
    #[serde(rename = "bodyTypeScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_type_scale: Option<f64>,
    #[serde(rename = "captchaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_id: Option<String>,
    #[serde(rename = "captchaProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_provider: Option<String>,
    #[serde(rename = "captchaToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_token: Option<String>,
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    #[serde(rename = "dataToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_token: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// \['Unknown' = 1, 'Male' = 2, 'Female' = 3\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<SignupRequestGender>,
    #[serde(rename = "headScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_scale: Option<f64>,
    #[serde(rename = "heightScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_scale: Option<f64>,
    #[serde(rename = "identityVerificationResultToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_verification_result_token: Option<String>,
    #[serde(rename = "isTosAgreementBoxChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tos_agreement_box_checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "otpSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_session: Option<OtpSessionModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "proportionScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion_scale: Option<f64>,
    #[serde(rename = "referralData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_data: Option<ReferralDataModel>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "widthScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_scale: Option<f64>,
}

/// `Roblox.Authentication.Api.Models.SignupResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SignupResponse {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(rename = "returnUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(rename = "starterPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starter_place_id: Option<i64>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Authentication.Api.Models.SocialAuthenticationDisconnectRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SocialAuthenticationDisconnectRequest {
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// `Roblox.Authentication.Api.Models.SocialProvidersResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SocialProvidersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<ProviderInfoModel>>,
}

/// `Roblox.Authentication.Api.Models.TwoStepVerificationMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStepVerificationMetadataResponse {
    #[serde(rename = "codeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_length: Option<i32>,
    #[serde(rename = "loadingImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_image_url: Option<String>,
    #[serde(rename = "supportUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

crate::api_enum! {
    int
    /// \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    TwoStepVerificationTicketRequestActionType {
    UnknownValue = 0 => "Unknown",
    Login = 1 => "Login",
    RobuxSpend = 2 => "RobuxSpend",
    ItemTrade = 3 => "ItemTrade",
    Resale = 4 => "Resale",
    PasswordReset = 5 => "PasswordReset",
    RevertAccount = 6 => "RevertAccount",
    Generic = 7 => "Generic",
    GenericWithRecoveryCodes = 8 => "GenericWithRecoveryCodes",
    }
}

/// `Roblox.Authentication.Api.Models.TwoStepVerificationTicketRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStepVerificationTicketRequest {
    /// \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<TwoStepVerificationTicketRequestActionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.TwoStepVerificationVerifyRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TwoStepVerificationVerifyRequest {
    /// \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<TwoStepVerificationTicketRequestActionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "rememberDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_device: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.UsernameChangeRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UsernameChangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.UsernameValidationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UsernameValidationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

crate::api_enum! {
    int
    /// \['ValidUsername' = 0, 'AlreadyInUseError' = 1, 'ModerationError' = 2, 'InvalidLengthError' = 3, 'StartsOrEndsWithUnderscoreError' = 4, 'TooManyUnderscoresError' = 5, 'ContainsSpacesError' = 6, 'InvalidCharactersError' = 7, 'ContainsPiiError' = 10, 'ContainsReservedUsernameError' = 12\]
    UsernameValidationResponseCode {
    ValidUsername = 0 => "ValidUsername",
    AlreadyInUseError = 1 => "AlreadyInUseError",
    ModerationError = 2 => "ModerationError",
    InvalidLengthError = 3 => "InvalidLengthError",
    StartsOrEndsWithUnderscoreError = 4 => "StartsOrEndsWithUnderscoreError",
    TooManyUnderscoresError = 5 => "TooManyUnderscoresError",
    ContainsSpacesError = 6 => "ContainsSpacesError",
    InvalidCharactersError = 7 => "InvalidCharactersError",
    ContainsPiiError = 10 => "ContainsPiiError",
    ContainsReservedUsernameError = 12 => "ContainsReservedUsernameError",
    }
}

/// `Roblox.Authentication.Api.Models.UsernameValidationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UsernameValidationResponse {
    /// \['ValidUsername' = 0, 'AlreadyInUseError' = 1, 'ModerationError' = 2, 'InvalidLengthError' = 3, 'StartsOrEndsWithUnderscoreError' = 4, 'TooManyUnderscoresError' = 5, 'ContainsSpacesError' = 6, 'InvalidCharactersError' = 7, 'ContainsPiiError' = 10, 'ContainsReservedUsernameError' = 12\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<UsernameValidationResponseCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// `Roblox.Authentication.Api.Models.UsernamesResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UsernamesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
}

/// `Roblox.Authentication.Api.Models.XboxUserModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct XboxUserModel {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.XboxCollectionsOfUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct XboxCollectionsOfUserResponse {
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<XboxUserModel>>,
}

/// `Roblox.Authentication.Api.Models.XboxConnectionModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct XboxConnectionModel {
    #[serde(rename = "hasConnectedXboxAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_connected_xbox_account: Option<bool>,
}

/// `Roblox.Authentication.Api.Models.XboxLoginConsecutiveDaysResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct XboxLoginConsecutiveDaysResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// `Roblox.Authentication.Api.Models.XboxTranslateRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct XboxTranslateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// `Roblox.Web.WebAPI.Models.ApiSuccessResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiSuccessResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// Query, header and form inputs for [`get_external_by_identity_provider_id_sso_oauth_callback`].
#[derive(Debug, Clone, Default)]
pub struct GetExternalByIdentityProviderIdSsoOauthCallbackParams {
    pub authorization_code: String,
    pub state: String,
}

/// Query, header and form inputs for [`get_passwords_validate`].
#[derive(Debug, Clone, Default)]
pub struct GetPasswordsValidateParams {
    pub username: String,
    pub password: String,
}

/// Query, header and form inputs for [`get_revert_account`].
#[derive(Debug, Clone, Default)]
pub struct GetRevertAccountParams {
    /// Ticket Guid to revert account.
    pub ticket: String,
}

/// Query, header and form inputs for [`get_usernames`].
#[derive(Debug, Clone, Default)]
pub struct GetUsernamesParams {
    /// The username
    pub username: Option<String>,
}

/// Query, header and form inputs for [`get_usernames_validate`].
#[derive(Debug, Clone, Default)]
pub struct GetUsernamesValidateParams {
    pub username: String,
    pub birthday: String,
    pub context: i32,
}

/// Query, header and form inputs for [`get_validators_email`].
#[derive(Debug, Clone, Default)]
pub struct GetValidatorsEmailParams {
    pub email: String,
}

/// Query, header and form inputs for [`get_validators_recommended_username_from_display_name`].
#[derive(Debug, Clone, Default)]
pub struct GetValidatorsRecommendedUsernameFromDisplayNameParams {
    pub display_name: String,
    pub birth_day: String,
}

/// Query, header and form inputs for [`get_validators_username`].
#[derive(Debug, Clone, Default)]
pub struct GetValidatorsUsernameParams {
    pub username: String,
    pub birth_day: String,
}

/// `GET /v1/account/pin`
///
/// Gets the account pin status. If the account pin is valid, this returns the time in seconds until when the account pin is unlocked.
pub fn get_account_pin_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/account/pin".to_string());
    Ok(r)
}

/// `GET /v1/account/pin`
///
/// Gets the account pin status. If the account pin is valid, this returns the time in seconds until when the account pin is unlocked.
pub async fn get_account_pin(client: &Client) -> Result<AccountPinStatusResponse> {
    client.send(get_account_pin_request()?).await?.json()
}

/// `POST /v1/account/pin`
///
/// Request to create the account pin.
pub fn post_account_pin_request(body: &AccountPinRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/account/pin".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/account/pin`
///
/// Request to create the account pin.
pub async fn post_account_pin(client: &Client, body: &AccountPinRequest) -> Result<ApiSuccessResponse> {
    client.send(post_account_pin_request(body)?).await?.json()
}

/// `PATCH /v1/account/pin`
///
/// Request made to update the account pin on the account.
pub fn patch_account_pin_request(body: &AccountPinRequest) -> Result<Request> {
    let mut r = Request::new(Method::PATCH, HOST, "/v1/account/pin".to_string());
    r.json(body)?;
    Ok(r)
}

/// `PATCH /v1/account/pin`
///
/// Request made to update the account pin on the account.
pub async fn patch_account_pin(client: &Client, body: &AccountPinRequest) -> Result<ApiSuccessResponse> {
    client.send(patch_account_pin_request(body)?).await?.json()
}

/// `DELETE /v1/account/pin`
///
/// Request for deletes the account pin from the account.
pub fn delete_account_pin_request(body: &AccountPinRequest) -> Result<Request> {
    let mut r = Request::new(Method::DELETE, HOST, "/v1/account/pin".to_string());
    r.json(body)?;
    Ok(r)
}

/// `DELETE /v1/account/pin`
///
/// Request for deletes the account pin from the account.
pub async fn delete_account_pin(client: &Client, body: &AccountPinRequest) -> Result<ApiSuccessResponse> {
    client.send(delete_account_pin_request(body)?).await?.json()
}

/// `POST /v1/account/pin/lock`
///
/// Request to locks the account which has an account pin enabled.
pub fn post_account_pin_lock_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/account/pin/lock".to_string());
    Ok(r)
}

/// `POST /v1/account/pin/lock`
///
/// Request to locks the account which has an account pin enabled.
pub async fn post_account_pin_lock(client: &Client) -> Result<ApiSuccessResponse> {
    client.send(post_account_pin_lock_request()?).await?.json()
}

/// `POST /v1/account/pin/unlock`
///
/// Requests to unlock the account pin.
pub fn post_account_pin_unlock_request(body: &AccountPinRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/account/pin/unlock".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/account/pin/unlock`
///
/// Requests to unlock the account pin.
pub async fn post_account_pin_unlock(
    client: &Client,
    body: &AccountPinRequest,
) -> Result<AccountPinResponse> {
    client.send(post_account_pin_unlock_request(body)?).await?.json()
}

/// `GET /v1/auth/metadata`
///
/// Gets Auth meta data
pub fn get_auth_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/auth/metadata".to_string());
    Ok(r)
}

/// `GET /v1/auth/metadata`
///
/// Gets Auth meta data
pub async fn get_auth_metadata(client: &Client) -> Result<AuthMetaDataResponse> {
    client.send(get_auth_metadata_request()?).await?.json()
}

/// `GET /v1/client-assertion`
///
/// Creates a client assertion to be used when generating an auth ticket.
pub fn get_client_assertion_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/client-assertion".to_string());
    Ok(r)
}

/// `GET /v1/client-assertion`
///
/// Creates a client assertion to be used when generating an auth ticket.
pub async fn get_client_assertion(client: &Client) -> Result<GetClientAssertionResponse> {
    client.send(get_client_assertion_request()?).await?.json()
}

/// `POST /v1/external/access`
///
/// Signs a user up for Roblox and links the account to the authenticated external provider ID.
pub fn post_external_access_request(body: &ExternalAccessRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/external/access".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/external/access`
///
/// Signs a user up for Roblox and links the account to the authenticated external provider ID.
pub async fn post_external_access(
    client: &Client,
    body: &ExternalAccessRequest,
) -> Result<ExternalIdentityAccessResponse> {
    client.send(post_external_access_request(body)?).await?.json()
}

/// `POST /v1/external/login`
///
/// Logs in a user to Roblox based on the user's authenticated external provider session
pub fn post_external_login_request(body: &ExternalLoginRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/external/login".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/external/login`
///
/// Logs in a user to Roblox based on the user's authenticated external provider session
pub async fn post_external_login(client: &Client, body: &ExternalLoginRequest) -> Result<()> {
    client.send(post_external_login_request(body)?).await?.unit()
}

/// `POST /v1/external/loginAndLink`
///
/// Logins in a user to Roblox, then links the Roblox account to the external provider ID
pub fn post_external_login_and_link_request(body: &ExternalLoginAndLinkRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/external/loginAndLink".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/external/loginAndLink`
///
/// Logins in a user to Roblox, then links the Roblox account to the external provider ID
pub async fn post_external_login_and_link(
    client: &Client,
    body: &ExternalLoginAndLinkRequest,
) -> Result<LoginResponse> {
    client
        .send(post_external_login_and_link_request(body)?)
        .await?
        .json()
}

/// `POST /v1/external/signup`
///
/// Signs a user up for Roblox and links the account to the authenticated external provider ID
pub fn post_external_signup_request(body: &ExternalSignupRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/external/signup".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/external/signup`
///
/// Signs a user up for Roblox and links the account to the authenticated external provider ID
pub async fn post_external_signup(client: &Client, body: &ExternalSignupRequest) -> Result<()> {
    client.send(post_external_signup_request(body)?).await?.unit()
}

/// `POST /v1/external/unlink`
///
/// Unlink the logged in Roblox account from the current external provider ID
pub fn post_external_unlink_request(body: &ExternalUnlinkRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/external/unlink".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/external/unlink`
///
/// Unlink the logged in Roblox account from the current external provider ID
pub async fn post_external_unlink(client: &Client, body: &ExternalUnlinkRequest) -> Result<()> {
    client.send(post_external_unlink_request(body)?).await?.unit()
}

/// `GET /v1/external/{identityProviderId}/sso/oauth/callback`
///
/// Callback function that external identity provider calls post user authentication.
pub fn get_external_by_identity_provider_id_sso_oauth_callback_request(
    identity_provider_id: &str,
    params: &GetExternalByIdentityProviderIdSsoOauthCallbackParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/external/{}/sso/oauth/callback",
            crate::rt::path_value(&identity_provider_id)
        ),
    );
    r.query("authorizationCode", &params.authorization_code);
    r.query("state", &params.state);
    Ok(r)
}

/// `GET /v1/external/{identityProviderId}/sso/oauth/callback`
///
/// Callback function that external identity provider calls post user authentication.
pub async fn get_external_by_identity_provider_id_sso_oauth_callback(
    client: &Client,
    identity_provider_id: &str,
    params: &GetExternalByIdentityProviderIdSsoOauthCallbackParams,
) -> Result<()> {
    client
        .send(get_external_by_identity_provider_id_sso_oauth_callback_request(
            identity_provider_id,
            params,
        )?)
        .await?
        .unit()
}

/// `GET /v1/external/{identityProviderId}/sso/oauth/init`
///
/// Signs a user up for Roblox and links the account to the authenticated external provider ID.
pub fn get_external_by_identity_provider_id_sso_oauth_init_request(
    identity_provider_id: &str,
) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!(
            "/v1/external/{}/sso/oauth/init",
            crate::rt::path_value(&identity_provider_id)
        ),
    );
    Ok(r)
}

/// `GET /v1/external/{identityProviderId}/sso/oauth/init`
///
/// Signs a user up for Roblox and links the account to the authenticated external provider ID.
pub async fn get_external_by_identity_provider_id_sso_oauth_init(
    client: &Client,
    identity_provider_id: &str,
) -> Result<()> {
    client
        .send(get_external_by_identity_provider_id_sso_oauth_init_request(
            identity_provider_id,
        )?)
        .await?
        .unit()
}

/// `POST /v1/identity-verification/login`
///
/// Endpoint for login with identity verification
pub fn post_identity_verification_login_request(body: &IdentityVerificationLoginRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/identity-verification/login".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/identity-verification/login`
///
/// Endpoint for login with identity verification
pub async fn post_identity_verification_login(
    client: &Client,
    body: &IdentityVerificationLoginRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_identity_verification_login_request(body)?)
        .await?
        .json()
}

/// `POST /v1/identity/initialize-login`
///
/// Initiates identifier-first login flow by returning a list of login methods for user(s).
pub fn post_identity_initialize_login_request(body: &InitializeLoginRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/identity/initialize-login".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/identity/initialize-login`
///
/// Initiates identifier-first login flow by returning a list of login methods for user(s).
pub async fn post_identity_initialize_login(
    client: &Client,
    body: &InitializeLoginRequest,
) -> Result<InitializeLoginResponse> {
    client
        .send(post_identity_initialize_login_request(body)?)
        .await?
        .json()
}

/// `POST /v1/login`
///
/// Authenticates a user.
pub fn post_login_request(body: &LoginRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/login".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/login`
///
/// Authenticates a user.
pub async fn post_login(client: &Client, body: &LoginRequest) -> Result<LoginResponse> {
    client.send(post_login_request(body)?).await?.json()
}

/// `POST /v1/logout`
///
/// Destroys the current authentication session.
pub fn post_logout_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/logout".to_string());
    Ok(r)
}

/// `POST /v1/logout`
///
/// Destroys the current authentication session.
pub async fn post_logout(client: &Client) -> Result<ApiEmptyResponseModel> {
    client.send(post_logout_request()?).await?.json()
}

/// `POST /v1/logoutfromallsessionsandreauthenticate`
///
/// Logs out user from all other sessions.
pub fn post_logoutfromallsessionsandreauthenticate_request(
    body: &LogoutFromAllSessionsAndReauthenticateRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/logoutfromallsessionsandreauthenticate".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/logoutfromallsessionsandreauthenticate`
///
/// Logs out user from all other sessions.
pub async fn post_logoutfromallsessionsandreauthenticate(
    client: &Client,
    body: &LogoutFromAllSessionsAndReauthenticateRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_logoutfromallsessionsandreauthenticate_request(body)?)
        .await?
        .json()
}

/// `GET /v1/metadata`
///
/// Get the metadata
pub fn get_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/metadata".to_string());
    Ok(r)
}

/// `GET /v1/metadata`
///
/// Get the metadata
pub async fn get_metadata(client: &Client) -> Result<MetadataResponse> {
    client.send(get_metadata_request()?).await?.json()
}

/// `POST /v1/passkey/DeleteCredentialBatch`
///
/// Disables a batch of credentials for the specified user.
pub fn post_passkey_delete_credential_batch_request(body: &DeletePasskeysRequest) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/passkey/DeleteCredentialBatch".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/passkey/DeleteCredentialBatch`
///
/// Disables a batch of credentials for the specified user.
pub async fn post_passkey_delete_credential_batch(
    client: &Client,
    body: &DeletePasskeysRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_passkey_delete_credential_batch_request(body)?)
        .await?
        .json()
}

/// `POST /v1/passkey/FinishRegistration`
///
/// Complete Passkey registration by providing credential creation options.
pub fn post_passkey_finish_registration_request(body: &FinishPasskeyRegistrationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/passkey/FinishRegistration".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/passkey/FinishRegistration`
///
/// Complete Passkey registration by providing credential creation options.
pub async fn post_passkey_finish_registration(
    client: &Client,
    body: &FinishPasskeyRegistrationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_passkey_finish_registration_request(body)?)
        .await?
        .json()
}

/// `POST /v1/passkey/ListCredentials`
///
/// List a user's registered passkeys.
pub fn post_passkey_list_credentials_request(body: &ListPasskeysRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/passkey/ListCredentials".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/passkey/ListCredentials`
///
/// List a user's registered passkeys.
pub async fn post_passkey_list_credentials(
    client: &Client,
    body: &ListPasskeysRequest,
) -> Result<ListPasskeyCredentialResponse> {
    client
        .send(post_passkey_list_credentials_request(body)?)
        .await?
        .json()
}

/// `POST /v1/passkey/StartAuthentication`
///
/// Provides a challenge for the Passkey to authenticate.
pub fn post_passkey_start_authentication_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/passkey/StartAuthentication".to_string());
    Ok(r)
}

/// `POST /v1/passkey/StartAuthentication`
///
/// Provides a challenge for the Passkey to authenticate.
pub async fn post_passkey_start_authentication(client: &Client) -> Result<StartAuthenticationResponse> {
    client
        .send(post_passkey_start_authentication_request()?)
        .await?
        .json()
}

/// `POST /v1/passkey/StartRegistration`
///
/// Initiates Passkey registration by providing credential creation options.
pub fn post_passkey_start_registration_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/passkey/StartRegistration".to_string());
    Ok(r)
}

/// `POST /v1/passkey/StartRegistration`
///
/// Initiates Passkey registration by providing credential creation options.
pub async fn post_passkey_start_registration(client: &Client) -> Result<StartPasskeyRegistrationResponse> {
    client
        .send(post_passkey_start_registration_request()?)
        .await?
        .json()
}

/// `POST /v1/passkey/start-authentication-by-user`
///
/// Initializes passkey authentication for the user(s) corresponding to the identifier provided.
pub fn post_passkey_start_authentication_by_user_request(
    body: &StartAuthenticationByUserRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/passkey/start-authentication-by-user".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/passkey/start-authentication-by-user`
///
/// Initializes passkey authentication for the user(s) corresponding to the identifier provided.
pub async fn post_passkey_start_authentication_by_user(
    client: &Client,
    body: &StartAuthenticationByUserRequest,
) -> Result<StartAuthenticationByUserResponse> {
    client
        .send(post_passkey_start_authentication_by_user_request(body)?)
        .await?
        .json()
}

/// `GET /v1/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub fn get_passwords_validate_request(params: &GetPasswordsValidateParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/passwords/validate".to_string());
    r.query("Username", &params.username);
    r.query("Password", &params.password);
    Ok(r)
}

/// `GET /v1/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub async fn get_passwords_validate(
    client: &Client,
    params: &GetPasswordsValidateParams,
) -> Result<PasswordValidationResponse> {
    client.send(get_passwords_validate_request(params)?).await?.json()
}

/// `POST /v1/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub fn post_passwords_validate_request(body: &PasswordValidationModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/passwords/validate".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub async fn post_passwords_validate(
    client: &Client,
    body: &PasswordValidationModel,
) -> Result<PasswordValidationResponse> {
    client.send(post_passwords_validate_request(body)?).await?.json()
}

/// `GET /v1/recovery/metadata`
///
/// Get metadata for forgot endpoints
pub fn get_recovery_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/recovery/metadata".to_string());
    Ok(r)
}

/// `GET /v1/recovery/metadata`
///
/// Get metadata for forgot endpoints
pub async fn get_recovery_metadata(client: &Client) -> Result<RecoveryMetadataResponse> {
    client.send(get_recovery_metadata_request()?).await?.json()
}

/// `GET /v1/revert/account`
///
/// Get Revert Account ticket info
pub fn get_revert_account_request(params: &GetRevertAccountParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/revert/account".to_string());
    r.query("ticket", &params.ticket);
    Ok(r)
}

/// `GET /v1/revert/account`
///
/// Get Revert Account ticket info
pub async fn get_revert_account(
    client: &Client,
    params: &GetRevertAccountParams,
) -> Result<RevertAccountInfoResponse> {
    client.send(get_revert_account_request(params)?).await?.json()
}

/// `POST /v1/revert/account`
///
/// Submit Revert Account Request
pub fn post_revert_account_request(body: &RevertAccountSubmitRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/revert/account".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/revert/account`
///
/// Submit Revert Account Request
pub async fn post_revert_account(
    client: &Client,
    body: &RevertAccountSubmitRequest,
) -> Result<LoginResponse> {
    client.send(post_revert_account_request(body)?).await?.json()
}

/// `POST /v1/signup`
///
/// Endpoint for signing up a new user
pub fn post_signup_request(body: &SignupRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/signup".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/signup`
///
/// Endpoint for signing up a new user
pub async fn post_signup(client: &Client, body: &SignupRequest) -> Result<SignupResponse> {
    client.send(post_signup_request(body)?).await?.json()
}

/// `GET /v1/social/connected-providers`
///
/// Get social network user information if the given social auth method is connected to current user.
pub fn get_social_connected_providers_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/social/connected-providers".to_string());
    Ok(r)
}

/// `GET /v1/social/connected-providers`
///
/// Get social network user information if the given social auth method is connected to current user.
pub async fn get_social_connected_providers(client: &Client) -> Result<SocialProvidersResponse> {
    client
        .send(get_social_connected_providers_request()?)
        .await?
        .json()
}

/// `POST /v1/social/{provider}/disconnect`
///
/// Removes the given social authentication method from current Roblox user if it is connected.
pub fn post_social_by_provider_disconnect_request(
    provider: &str,
    body: &SocialAuthenticationDisconnectRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!("/v1/social/{}/disconnect", crate::rt::path_value(&provider)),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/social/{provider}/disconnect`
///
/// Removes the given social authentication method from current Roblox user if it is connected.
pub async fn post_social_by_provider_disconnect(
    client: &Client,
    provider: &str,
    body: &SocialAuthenticationDisconnectRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_social_by_provider_disconnect_request(provider, body)?)
        .await?
        .json()
}

/// `GET /v1/twostepverification/metadata`
///
/// Get metadata for two step verification
pub fn get_twostepverification_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/twostepverification/metadata".to_string());
    Ok(r)
}

/// `GET /v1/twostepverification/metadata`
///
/// Get metadata for two step verification
pub async fn get_twostepverification_metadata(
    client: &Client,
) -> Result<TwoStepVerificationMetadataResponse> {
    client
        .send(get_twostepverification_metadata_request()?)
        .await?
        .json()
}

/// `POST /v1/twostepverification/resend`
///
/// Resends a two step verification code.
pub fn post_twostepverification_resend_request(body: &TwoStepVerificationTicketRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/twostepverification/resend".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/twostepverification/resend`
///
/// Resends a two step verification code.
pub async fn post_twostepverification_resend(
    client: &Client,
    body: &TwoStepVerificationTicketRequest,
) -> Result<TwoStepVerificationSentResponse> {
    client
        .send(post_twostepverification_resend_request(body)?)
        .await?
        .json()
}

/// `POST /v1/twostepverification/verify`
///
/// Verifies a two step verification code.
pub fn post_twostepverification_verify_request(body: &TwoStepVerificationVerifyRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/twostepverification/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/twostepverification/verify`
///
/// Verifies a two step verification code.
pub async fn post_twostepverification_verify(
    client: &Client,
    body: &TwoStepVerificationVerifyRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_twostepverification_verify_request(body)?)
        .await?
        .json()
}

/// `POST /v1/user/passwords/change`
///
/// Changes the password for the authenticated user.
///
/// The current password is needed for verification that the password can be changed.
pub fn post_user_passwords_change_request(body: &PasswordChangeModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/user/passwords/change".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/user/passwords/change`
///
/// Changes the password for the authenticated user.
///
/// The current password is needed for verification that the password can be changed.
pub async fn post_user_passwords_change(
    client: &Client,
    body: &PasswordChangeModel,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_user_passwords_change_request(body)?)
        .await?
        .json()
}

/// `POST /v1/username`
///
/// Change the user's username
pub fn post_username_request(body: &UsernameChangeRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/username".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/username`
///
/// Change the user's username
pub async fn post_username(client: &Client, body: &UsernameChangeRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_username_request(body)?).await?.json()
}

/// `GET /v1/usernames`
///
/// Gets a list of existing usernames on Roblox based on the query parameters
///
/// This endpoint can be expanded in the future to include other query parameters such as "startsWith"
pub fn get_usernames_request(params: &GetUsernamesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/usernames".to_string());
    r.query("username", &params.username);
    Ok(r)
}

/// `GET /v1/usernames`
///
/// Gets a list of existing usernames on Roblox based on the query parameters
///
/// This endpoint can be expanded in the future to include other query parameters such as "startsWith"
pub async fn get_usernames(client: &Client, params: &GetUsernamesParams) -> Result<UsernamesResponse> {
    client.send(get_usernames_request(params)?).await?.json()
}

/// `POST /v1/usernames/recover`
///
/// Sends an email of all accounts belonging to an email
pub fn post_usernames_recover_request(body: &RecoverUsernameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/usernames/recover".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/usernames/recover`
///
/// Sends an email of all accounts belonging to an email
pub async fn post_usernames_recover(
    client: &Client,
    body: &RecoverUsernameRequest,
) -> Result<RecoverUsernameResponse> {
    client.send(post_usernames_recover_request(body)?).await?.json()
}

/// `GET /v1/usernames/validate`
///
/// Checks if a username is valid.
pub fn get_usernames_validate_request(params: &GetUsernamesValidateParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/usernames/validate".to_string());
    r.query("Username", &params.username);
    r.query("Birthday", &params.birthday);
    r.query("Context", &params.context);
    Ok(r)
}

/// `GET /v1/usernames/validate`
///
/// Checks if a username is valid.
pub async fn get_usernames_validate(
    client: &Client,
    params: &GetUsernamesValidateParams,
) -> Result<UsernameValidationResponse> {
    client.send(get_usernames_validate_request(params)?).await?.json()
}

/// `POST /v1/usernames/validate`
///
/// Checks if a username is valid.
pub fn post_usernames_validate_request(body: &UsernameValidationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/usernames/validate".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/usernames/validate`
///
/// Checks if a username is valid.
pub async fn post_usernames_validate(
    client: &Client,
    body: &UsernameValidationRequest,
) -> Result<UsernameValidationResponse> {
    client.send(post_usernames_validate_request(body)?).await?.json()
}

/// `GET /v1/validators/email`
///
/// Tries to check if an email is valid
pub fn get_validators_email_request(params: &GetValidatorsEmailParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/validators/email".to_string());
    r.query("Email", &params.email);
    Ok(r)
}

/// `GET /v1/validators/email`
///
/// Tries to check if an email is valid
pub async fn get_validators_email(
    client: &Client,
    params: &GetValidatorsEmailParams,
) -> Result<EmailValidationResponse> {
    client.send(get_validators_email_request(params)?).await?.json()
}

/// `GET /v1/validators/recommendedUsernameFromDisplayName`
///
/// Validates the given display name, and if valid, will convert it to a valid username and return suggested username(s) if available.
pub fn get_validators_recommended_username_from_display_name_request(
    params: &GetValidatorsRecommendedUsernameFromDisplayNameParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v1/validators/recommendedUsernameFromDisplayName".to_string(),
    );
    r.query("DisplayName", &params.display_name);
    r.query("BirthDay", &params.birth_day);
    Ok(r)
}

/// `GET /v1/validators/recommendedUsernameFromDisplayName`
///
/// Validates the given display name, and if valid, will convert it to a valid username and return suggested username(s) if available.
pub async fn get_validators_recommended_username_from_display_name(
    client: &Client,
    params: &GetValidatorsRecommendedUsernameFromDisplayNameParams,
) -> Result<RecommendedUsernameResponse> {
    client
        .send(get_validators_recommended_username_from_display_name_request(
            params,
        )?)
        .await?
        .json()
}

/// `POST /v1/validators/recommendedUsernameFromDisplayName`
///
/// Validates the given display name, and if valid, will convert it to a valid username and return suggested username(s) if available. This is a POST request and explicitly does not receive the parameter values from the query
pub fn post_validators_recommended_username_from_display_name_request(
    body: &RecommendedUsernameFromDisplayNameRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v1/validators/recommendedUsernameFromDisplayName".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/validators/recommendedUsernameFromDisplayName`
///
/// Validates the given display name, and if valid, will convert it to a valid username and return suggested username(s) if available. This is a POST request and explicitly does not receive the parameter values from the query
pub async fn post_validators_recommended_username_from_display_name(
    client: &Client,
    body: &RecommendedUsernameFromDisplayNameRequest,
) -> Result<RecommendedUsernameResponse> {
    client
        .send(post_validators_recommended_username_from_display_name_request(
            body,
        )?)
        .await?
        .json()
}

/// `GET /v1/validators/username`
///
/// Tries to get a valid username if the current username is taken
pub fn get_validators_username_request(params: &GetValidatorsUsernameParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/validators/username".to_string());
    r.query("Username", &params.username);
    r.query("BirthDay", &params.birth_day);
    Ok(r)
}

/// `GET /v1/validators/username`
///
/// Tries to get a valid username if the current username is taken
pub async fn get_validators_username(
    client: &Client,
    params: &GetValidatorsUsernameParams,
) -> Result<RecommendedUsernameResponse> {
    client
        .send(get_validators_username_request(params)?)
        .await?
        .json()
}

/// `POST /v1/validators/username`
///
/// Tries to get a valid username if the current username is taken This is a POST request and explicitly does not receive the parameter values from the query
pub fn post_validators_username_request(body: &RecommendedUsernameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/validators/username".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/validators/username`
///
/// Tries to get a valid username if the current username is taken This is a POST request and explicitly does not receive the parameter values from the query
pub async fn post_validators_username(
    client: &Client,
    body: &RecommendedUsernameRequest,
) -> Result<RecommendedUsernameResponse> {
    client.send(post_validators_username_request(body)?).await?.json()
}

/// `GET /v1/xbox/connection`
///
/// Check if the current user has an Xbox connected.
pub fn get_xbox_connection_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v1/xbox/connection".to_string());
    Ok(r)
}

/// `GET /v1/xbox/connection`
///
/// Check if the current user has an Xbox connected.
pub async fn get_xbox_connection(client: &Client) -> Result<XboxConnectionModel> {
    client.send(get_xbox_connection_request()?).await?.json()
}

/// `POST /v1/xbox/disconnect`
///
/// Unlink the current ROBLOX account from the Xbox live account.
pub fn post_xbox_disconnect_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v1/xbox/disconnect".to_string());
    Ok(r)
}

/// `POST /v1/xbox/disconnect`
///
/// Unlink the current ROBLOX account from the Xbox live account.
pub async fn post_xbox_disconnect(client: &Client) -> Result<ApiSuccessResponse> {
    client.send(post_xbox_disconnect_request()?).await?.json()
}

/// `GET /v1/xbox/get-login-consecutive-days`
///
/// Get the consecutive days the xbox user has been logged in.
pub fn get_xbox_get_login_consecutive_days_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v1/xbox/get-login-consecutive-days".to_string(),
    );
    Ok(r)
}

/// `GET /v1/xbox/get-login-consecutive-days`
///
/// Get the consecutive days the xbox user has been logged in.
pub async fn get_xbox_get_login_consecutive_days(
    client: &Client,
) -> Result<XboxLoginConsecutiveDaysResponse> {
    client
        .send(get_xbox_get_login_consecutive_days_request()?)
        .await?
        .json()
}

/// `POST /v1/xbox/translate`
///
/// Translate the xbox user to roblox user.
pub fn post_xbox_translate_request(body: &XboxTranslateRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v1/xbox/translate".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/xbox/translate`
///
/// Translate the xbox user to roblox user.
pub async fn post_xbox_translate(
    client: &Client,
    body: &XboxTranslateRequest,
) -> Result<XboxCollectionsOfUserResponse> {
    client.send(post_xbox_translate_request(body)?).await?.json()
}
