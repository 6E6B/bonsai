//! `auth.roblox.com` v2 — generated from `vendor/roblox-api-tracker/auth/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "auth.roblox.com";

/// `Roblox.Authentication.Api.Models.AuthMetaDataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AuthMetaDataResponse {
    #[serde(rename = "cookieLawNoticeTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_law_notice_timeout: Option<i32>,
}

/// `Roblox.Web.Responses.Users.LegacyUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LegacyUserResponse {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// `Roblox.Authentication.Api.Models.ForgotPasswordUserResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ForgotPasswordUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<LegacyUserResponse>,
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

/// `Roblox.Authentication.Api.Models.PasswordResetMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordResetMetadataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<LegacyUserResponse>>,
}

crate::api_enum! {
    int
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    PasswordResetModelTargetType {
    Email = 0 => "Email",
    PhoneNumber = 1 => "PhoneNumber",
    }
}

/// `Roblox.Authentication.Api.Models.PasswordResetModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordResetModel {
    #[serde(rename = "accountBlob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_blob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordRepeated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_repeated: Option<String>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<PasswordResetModelTargetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    #[serde(rename = "twoStepVerificationChallengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_verification_challenge_id: Option<String>,
    #[serde(rename = "twoStepVerificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_verification_token: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Authentication.Api.Models.PasswordResetVerificationRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordResetVerificationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<PasswordResetModelTargetType>,
}

/// `Roblox.Authentication.Api.Models.PasswordResetVerificationResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordResetVerificationResponse {
    #[serde(rename = "userTickets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_tickets: Option<Vec<ForgotPasswordUserResponse>>,
}

/// `Roblox.Authentication.Api.Models.PasswordStatusResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PasswordStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
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

/// `Roblox.Authentication.Api.Models.RecoverUsernameRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecoverUsernameRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<PasswordResetModelTargetType>,
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

/// `Roblox.Authentication.Api.Models.SendResetPasswordRequest`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendResetPasswordRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// \['Email' = 0, 'PhoneNumber' = 1\]
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<PasswordResetModelTargetType>,
}

crate::api_enum! {
    int
    /// \['Sms' = 0, 'Email' = 1\]
    SendResetPasswordResponseTransmissionType {
    Sms = 0 => "Sms",
    Email = 1 => "Email",
    }
}

/// `Roblox.Authentication.Api.Models.SendResetPasswordResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendResetPasswordResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    /// \['Sms' = 0, 'Email' = 1\]
    #[serde(rename = "transmissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_type: Option<SendResetPasswordResponseTransmissionType>,
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

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// Query, header and form inputs for [`get_passwords_reset`].
#[derive(Debug, Clone, Default)]
pub struct GetPasswordsResetParams {
    pub target_type: i32,
    pub ticket: String,
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

/// `GET /v2/auth/metadata`
///
/// Gets Auth meta data
pub fn get_auth_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/auth/metadata".to_string());
    Ok(r)
}

/// `GET /v2/auth/metadata`
///
/// Gets Auth meta data
pub async fn get_auth_metadata(client: &Client) -> Result<AuthMetaDataResponse> {
    client.send(get_auth_metadata_request()?).await?.json()
}

/// `POST /v2/identity-verification/login`
///
/// Endpoint for login with identity verification
pub fn post_identity_verification_login_request(body: &IdentityVerificationLoginRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/identity-verification/login".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/identity-verification/login`
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

/// `POST /v2/login`
///
/// Authenticates a user.
pub fn post_login_request(body: &LoginRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/login".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/login`
///
/// Authenticates a user.
pub async fn post_login(client: &Client, body: &LoginRequest) -> Result<LoginResponse> {
    client.send(post_login_request(body)?).await?.json()
}

/// `POST /v2/logout`
///
/// Destroys the current authentication session.
pub fn post_logout_request() -> Result<Request> {
    let r = Request::new(Method::POST, HOST, "/v2/logout".to_string());
    Ok(r)
}

/// `POST /v2/logout`
///
/// Destroys the current authentication session.
pub async fn post_logout(client: &Client) -> Result<ApiEmptyResponseModel> {
    client.send(post_logout_request()?).await?.json()
}

/// `POST /v2/logoutfromallsessionsandreauthenticate`
///
/// Logs out user from all other sessions.
pub fn post_logoutfromallsessionsandreauthenticate_request(
    body: &LogoutFromAllSessionsAndReauthenticateRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/logoutfromallsessionsandreauthenticate".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/logoutfromallsessionsandreauthenticate`
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

/// `GET /v2/metadata`
///
/// Get the metadata
pub fn get_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/metadata".to_string());
    Ok(r)
}

/// `GET /v2/metadata`
///
/// Get the metadata
pub async fn get_metadata(client: &Client) -> Result<MetadataResponse> {
    client.send(get_metadata_request()?).await?.json()
}

/// `GET /v2/passwords/current-status`
///
/// Returns password status for current user.
pub fn get_passwords_current_status_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/passwords/current-status".to_string());
    Ok(r)
}

/// `GET /v2/passwords/current-status`
///
/// Returns password status for current user.
pub async fn get_passwords_current_status(client: &Client) -> Result<PasswordStatusResponse> {
    client.send(get_passwords_current_status_request()?).await?.json()
}

/// `GET /v2/passwords/reset`
///
/// Gets metadata needed for the password reset view.
pub fn get_passwords_reset_request(params: &GetPasswordsResetParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/passwords/reset".to_string());
    r.query("TargetType", &params.target_type);
    r.query("Ticket", &params.ticket);
    Ok(r)
}

/// `GET /v2/passwords/reset`
///
/// Gets metadata needed for the password reset view.
pub async fn get_passwords_reset(
    client: &Client,
    params: &GetPasswordsResetParams,
) -> Result<PasswordResetMetadataResponse> {
    client.send(get_passwords_reset_request(params)?).await?.json()
}

/// `POST /v2/passwords/reset`
///
/// Resets a password for a user that belongs to the password reset ticket.
///
/// This will log the user out of all sessions and re-authenticate.
pub fn post_passwords_reset_request(body: &PasswordResetModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/passwords/reset".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/passwords/reset`
///
/// Resets a password for a user that belongs to the password reset ticket.
///
/// This will log the user out of all sessions and re-authenticate.
pub async fn post_passwords_reset(client: &Client, body: &PasswordResetModel) -> Result<LoginResponse> {
    client.send(post_passwords_reset_request(body)?).await?.json()
}

/// `POST /v2/passwords/reset/send`
///
/// Sends a password reset email or challenge to the specified target.
///
/// Phone target must be a csv with 3 values: "internationalPrefixNumber,nationalNumber,countryCode"
pub fn post_passwords_reset_send_request(body: &SendResetPasswordRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/passwords/reset/send".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/passwords/reset/send`
///
/// Sends a password reset email or challenge to the specified target.
///
/// Phone target must be a csv with 3 values: "internationalPrefixNumber,nationalNumber,countryCode"
pub async fn post_passwords_reset_send(
    client: &Client,
    body: &SendResetPasswordRequest,
) -> Result<SendResetPasswordResponse> {
    client
        .send(post_passwords_reset_send_request(body)?)
        .await?
        .json()
}

/// `POST /v2/passwords/reset/verify`
///
/// Verifies a password reset challenge solution.
pub fn post_passwords_reset_verify_request(body: &PasswordResetVerificationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/passwords/reset/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/passwords/reset/verify`
///
/// Verifies a password reset challenge solution.
pub async fn post_passwords_reset_verify(
    client: &Client,
    body: &PasswordResetVerificationRequest,
) -> Result<PasswordResetVerificationResponse> {
    client
        .send(post_passwords_reset_verify_request(body)?)
        .await?
        .json()
}

/// `GET /v2/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub fn get_passwords_validate_request(params: &GetPasswordsValidateParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/passwords/validate".to_string());
    r.query("Username", &params.username);
    r.query("Password", &params.password);
    Ok(r)
}

/// `GET /v2/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub async fn get_passwords_validate(
    client: &Client,
    params: &GetPasswordsValidateParams,
) -> Result<PasswordValidationResponse> {
    client.send(get_passwords_validate_request(params)?).await?.json()
}

/// `POST /v2/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub fn post_passwords_validate_request(body: &PasswordValidationModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/passwords/validate".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/passwords/validate`
///
/// Endpoint for checking if a password is valid.
pub async fn post_passwords_validate(
    client: &Client,
    body: &PasswordValidationModel,
) -> Result<PasswordValidationResponse> {
    client.send(post_passwords_validate_request(body)?).await?.json()
}

/// `GET /v2/recovery/metadata`
///
/// Get metadata for forgot endpoints
pub fn get_recovery_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/recovery/metadata".to_string());
    Ok(r)
}

/// `GET /v2/recovery/metadata`
///
/// Get metadata for forgot endpoints
pub async fn get_recovery_metadata(client: &Client) -> Result<RecoveryMetadataResponse> {
    client.send(get_recovery_metadata_request()?).await?.json()
}

/// `GET /v2/revert/account`
///
/// Get Revert Account ticket info
pub fn get_revert_account_request(params: &GetRevertAccountParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/revert/account".to_string());
    r.query("ticket", &params.ticket);
    Ok(r)
}

/// `GET /v2/revert/account`
///
/// Get Revert Account ticket info
pub async fn get_revert_account(
    client: &Client,
    params: &GetRevertAccountParams,
) -> Result<RevertAccountInfoResponse> {
    client.send(get_revert_account_request(params)?).await?.json()
}

/// `POST /v2/revert/account`
///
/// Submit Revert Account Request
pub fn post_revert_account_request(body: &RevertAccountSubmitRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/revert/account".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/revert/account`
///
/// Submit Revert Account Request
pub async fn post_revert_account(
    client: &Client,
    body: &RevertAccountSubmitRequest,
) -> Result<LoginResponse> {
    client.send(post_revert_account_request(body)?).await?.json()
}

/// `POST /v2/signup`
///
/// Endpoint for signing up a new user
pub fn post_signup_request(body: &SignupRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/signup".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/signup`
///
/// Endpoint for signing up a new user
pub async fn post_signup(client: &Client, body: &SignupRequest) -> Result<SignupResponse> {
    client.send(post_signup_request(body)?).await?.json()
}

/// `GET /v2/twostepverification/metadata`
///
/// Get metadata for two step verification
pub fn get_twostepverification_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/twostepverification/metadata".to_string());
    Ok(r)
}

/// `GET /v2/twostepverification/metadata`
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

/// `POST /v2/twostepverification/resend`
///
/// Resends a two step verification code.
pub fn post_twostepverification_resend_request(body: &TwoStepVerificationTicketRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/twostepverification/resend".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/twostepverification/resend`
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

/// `POST /v2/twostepverification/verify`
///
/// Verifies a two step verification code.
pub fn post_twostepverification_verify_request(body: &TwoStepVerificationVerifyRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/twostepverification/verify".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/twostepverification/verify`
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

/// `POST /v2/user/passwords/change`
///
/// Changes the password for the authenticated user.
///
/// The current password is needed for verification that the password can be changed.
pub fn post_user_passwords_change_request(body: &PasswordChangeModel) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/user/passwords/change".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/user/passwords/change`
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

/// `POST /v2/username`
///
/// Change the user's username
pub fn post_username_request(body: &UsernameChangeRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/username".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/username`
///
/// Change the user's username
pub async fn post_username(client: &Client, body: &UsernameChangeRequest) -> Result<ApiEmptyResponseModel> {
    client.send(post_username_request(body)?).await?.json()
}

/// `GET /v2/usernames`
///
/// Gets a list of existing usernames on Roblox based on the query parameters
///
/// This endpoint can be expanded in the future to include other query parameters such as "startsWith"
pub fn get_usernames_request(params: &GetUsernamesParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/usernames".to_string());
    r.query("username", &params.username);
    Ok(r)
}

/// `GET /v2/usernames`
///
/// Gets a list of existing usernames on Roblox based on the query parameters
///
/// This endpoint can be expanded in the future to include other query parameters such as "startsWith"
pub async fn get_usernames(client: &Client, params: &GetUsernamesParams) -> Result<UsernamesResponse> {
    client.send(get_usernames_request(params)?).await?.json()
}

/// `POST /v2/usernames/recover`
///
/// Sends an email of all accounts belonging to an email
pub fn post_usernames_recover_request(body: &RecoverUsernameRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/usernames/recover".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/usernames/recover`
///
/// Sends an email of all accounts belonging to an email
pub async fn post_usernames_recover(
    client: &Client,
    body: &RecoverUsernameRequest,
) -> Result<RecoverUsernameResponse> {
    client.send(post_usernames_recover_request(body)?).await?.json()
}

/// `GET /v2/usernames/validate`
///
/// Checks if a username is valid.
pub fn get_usernames_validate_request(params: &GetUsernamesValidateParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/usernames/validate".to_string());
    r.query("Username", &params.username);
    r.query("Birthday", &params.birthday);
    r.query("Context", &params.context);
    Ok(r)
}

/// `GET /v2/usernames/validate`
///
/// Checks if a username is valid.
pub async fn get_usernames_validate(
    client: &Client,
    params: &GetUsernamesValidateParams,
) -> Result<UsernameValidationResponse> {
    client.send(get_usernames_validate_request(params)?).await?.json()
}

/// `POST /v2/usernames/validate`
///
/// Checks if a username is valid.
pub fn post_usernames_validate_request(body: &UsernameValidationRequest) -> Result<Request> {
    let mut r = Request::new(Method::POST, HOST, "/v2/usernames/validate".to_string());
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/usernames/validate`
///
/// Checks if a username is valid.
pub async fn post_usernames_validate(
    client: &Client,
    body: &UsernameValidationRequest,
) -> Result<UsernameValidationResponse> {
    client.send(post_usernames_validate_request(body)?).await?.json()
}
