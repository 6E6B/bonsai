//! `twostepverification.roblox.com` v1 — generated from `vendor/roblox-api-tracker/twostepverification/v1.json`.
//!
//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "twostepverification.roblox.com";

/// `Roblox.TwoStepVerification.Api.ClearRecoveryCodesRequest`
///
/// Request information needed to clear existing recovery codes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ClearRecoveryCodesRequest {
    /// The user's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.DisableSecurityKeyRequest`
///
/// Request information needed to disable a list of security keys.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisableSecurityKeyRequest {
    /// An array of IDs of credentials to be deleted. Preferred over nicknames.
    #[serde(rename = "credentialIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_i_ds: Option<Vec<String>>,
    /// An array of nicknames of credentials to be deleted.
    #[serde(rename = "credentialNicknames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_nicknames: Option<Vec<String>>,
}

/// `Roblox.TwoStepVerification.Api.DisableTwoStepVerificationRequest`
///
/// Request information needed to disable two step verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DisableTwoStepVerificationRequest {
    /// The user's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// A re-authentication token redeemable for any password check.
    #[serde(rename = "reauthenticationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reauthentication_token: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.EnableAuthenticatorResponse`
///
/// Response parameters for initiating enabling authenticator-based two step verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnableAuthenticatorResponse {
    /// The manual entry key to input into the authenticator app.
    #[serde(rename = "manualEntryKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_entry_key: Option<String>,
    /// The Url to the QR code image to scan into the authenticator app.
    #[serde(rename = "qrCodeImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_image_url: Option<String>,
    /// The setup token for turning on authenticator.
    #[serde(rename = "setupToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_token: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.EnableSecurityKeyResponse`
///
/// Response parameters for initiating enabling security key-based two step verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnableSecurityKeyResponse {
    /// The credential registration options for the hardware key.
    #[serde(rename = "creationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_options: Option<String>,
    /// The session of the registration attempt.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.Models.Request.SecureAuthenticationIntentModel`
///
/// Model describing secure auth intent.
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

/// `Roblox.TwoStepVerification.Api.EnableTwoStepVerificationRequest`
///
/// Request parameters for enabling two step verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnableTwoStepVerificationRequest {
    /// The user's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
}

/// `Roblox.TwoStepVerification.Api.EnableVerifyAuthenticatorRequest`
///
/// Request parameters for authenticator enabling-verify.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnableVerifyAuthenticatorRequest {
    /// The code from the authenticator app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The user's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "secureAuthenticationIntent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_authentication_intent: Option<SecureAuthenticationIntentModel>,
    /// The setup token from the enable authenticator request.
    #[serde(rename = "setupToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_token: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.EnableVerifyAuthenticatorResponse`
///
/// Response parameters for finishing enabling authenticator-based two-step verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnableVerifyAuthenticatorResponse {
    /// Recovery codes automatically generated for the user (when applicable; the user should not already have a set of recovery codes).
    #[serde(rename = "recoveryCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_codes: Option<Vec<String>>,
}

/// `Roblox.TwoStepVerification.Api.EnableVerifySecurityKeyRequest`
///
/// Request information needed to complete the registration of a security key.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EnableVerifySecurityKeyRequest {
    /// The hardware key's response.
    #[serde(rename = "attestationResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_response: Option<String>,
    /// The nickname of the new credential.
    #[serde(rename = "credentialNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_nickname: Option<String>,
    /// The session of the registration attempt.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.SecurityKeyCredential`
///
/// Credential information that includes its nickname and any additional metadata.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SecurityKeyCredential {
    /// Foreign key reference for this credential in the security-key-credentials database
    #[serde(rename = "credentialID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    /// Nickname the user has chosen for this credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.ListSecurityKeyResponse`
///
/// Response parameters for listing all credentials under a user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ListSecurityKeyResponse {
    /// An array of credentials for a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<SecurityKeyCredential>>,
}

/// `Roblox.TwoStepVerification.Api.MetadataResponse`
///
/// Two step verification system metadata.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MetadataResponse {
    /// Number of characters in an authenticator-based two step verification code.
    #[serde(rename = "authenticatorCodeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_code_length: Option<i32>,
    /// Address of the help site provided to users to help them with authenticator.
    #[serde(rename = "authenticatorHelpSiteAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_help_site_address: Option<String>,
    /// Authenticator QR code image dimensions.
    #[serde(rename = "authenticatorQrCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_qr_code_size: Option<String>,
    /// Number of characters in an email-based two step verification code.
    #[serde(rename = "emailCodeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_code_length: Option<i32>,
    /// Whether to expose 2SV recovery through the 2SV challenge modal.
    #[serde(rename = "is2svRecoveryEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is2sv_recovery_enabled: Option<bool>,
    /// Whether Android security keys is enabled.
    #[serde(rename = "isAndroidSecurityKeyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_android_security_key_enabled: Option<bool>,
    /// Whether or not someone can enable authenticator with just a verified phone number.
    #[serde(rename = "isAuthenticatorWithVerifiedPhoneEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_authenticator_with_verified_phone_enabled: Option<bool>,
    /// Whether the delayed UI is enabled.
    #[serde(rename = "isDelayedUiEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delayed_ui_enabled: Option<bool>,
    /// Whether the EPP recovery codes UI is enabled.
    #[serde(rename = "isEppRecoveryCodesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_epp_recovery_codes_enabled: Option<bool>,
    /// Whether the EPP UI is enabled.
    #[serde(rename = "isEppUIEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_epp_ui_enabled: Option<bool>,
    /// Whether or not a password is required for making updates to recovery codes.
    #[serde(rename = "isPasswordRequiredForChangingRecoveryCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_changing_recovery_codes: Option<bool>,
    /// Whether or not a password is required for disabling authenticator.
    #[serde(rename = "isPasswordRequiredForDisablingAuthenticator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_disabling_authenticator: Option<bool>,
    /// Whether or not a password is required for disabling email 2sv.
    #[serde(rename = "isPasswordRequiredForDisablingEmailTwoStepVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_disabling_email_two_step_verification: Option<bool>,
    /// Whether or not a password is required for disabling SMS 2SV.
    #[serde(rename = "isPasswordRequiredForDisablingSms2SV")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_disabling_sms2_sv: Option<bool>,
    /// Whether or not a password is required for enabling authenticator.
    #[serde(rename = "isPasswordRequiredForEnablingAuthenticator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_enabling_authenticator: Option<bool>,
    /// Whether or not a password is required for enabling email 2sv.
    #[serde(rename = "isPasswordRequiredForEnablingEmailTwoStepVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_enabling_email_two_step_verification: Option<bool>,
    /// Whether or not a password is required for enabling Security Key 2SV.
    #[serde(rename = "isPasswordRequiredForEnablingSecurityKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_enabling_security_key: Option<bool>,
    /// Whether or not a password is required for enabling SMS 2SV.
    #[serde(rename = "isPasswordRequiredForEnablingSms2SV")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_password_required_for_enabling_sms2_sv: Option<bool>,
    /// Whether recovery code generation is attempted upon completion of authenticator setup.
    #[serde(rename = "isRecoveryCodeGenerationForAuthenticatorSetupEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recovery_code_generation_for_authenticator_setup_enabled: Option<bool>,
    /// Whether security keys on all platforms is enabled.
    #[serde(rename = "isSecurityKeyOnAllPlatformsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_security_key_on_all_platforms_enabled: Option<bool>,
    /// Whether or not security key two step verification is available for the user.
    #[serde(rename = "isSecurityKeyTwoStepVerificationAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_security_key_two_step_verification_available: Option<bool>,
    /// Whether the settings tab redesign is enabled.
    #[serde(rename = "isSettingsTabRedesignEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_settings_tab_redesign_enabled: Option<bool>,
    /// Whether or not the frontend should enforce single method logic.
    #[serde(rename = "isSingleMethodEnforcementEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_single_method_enforcement_enabled: Option<bool>,
    /// Whether or not sms two step verification is available for the user.
    #[serde(rename = "isSmsTwoStepVerificationAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sms_two_step_verification_available: Option<bool>,
    /// Whether or not a 2sv method has to be enabled to require password when enabling authenticator.
    #[serde(rename = "isTwoStepEnabledRequiredForAuthenticatorPasswordRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_two_step_enabled_required_for_authenticator_password_requirement: Option<bool>,
    /// Whether or not a 2sv method has to be enabled to require password when enabling email two step verification.
    #[serde(rename = "isTwoStepEnabledRequiredForEmailPasswordRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_two_step_enabled_required_for_email_password_requirement: Option<bool>,
    /// Whether the user is O13.
    #[serde(rename = "isUserU13")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_u13: Option<bool>,
    /// Whether or not we're using the new 2sv webview component or the manual pop up
    #[serde(rename = "isUsingTwoStepWebviewComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_using_two_step_webview_component: Option<bool>,
    /// The masked email for the authenticated user. Typically used in Email 2SV challenges after the challenge has been verified to match the target user.
    #[serde(rename = "maskedUserEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_user_email: Option<String>,
    /// Whether users should receive additional warnings when disabling 2SV.
    #[serde(rename = "receiveWarningsOnDisableTwoStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_warnings_on_disable_two_step: Option<bool>,
    /// The enum representing which experiment group the user is in.
    #[serde(rename = "twoStepCopyTextEnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_copy_text_enrollment_status: Option<i32>,
    /// Whether or not two step verification is globally enabled.
    #[serde(rename = "twoStepVerificationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_step_verification_enabled: Option<bool>,
}

/// `Roblox.TwoStepVerification.Api.RecoveryCodesStatusResponse`
///
/// The response for getting the status of recovery codes for a user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecoveryCodesStatusResponse {
    /// The number of unused recovery codes the user has available.
    #[serde(rename = "activeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_count: Option<i32>,
    /// The date time the recovery codes were generated at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.RegenerateRecoveryCodesRequest`
///
/// Request information needed to regenerate recovery codes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RegenerateRecoveryCodesRequest {
    /// The user's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.RegenerateRecoveryCodesResponse`
///
/// The response for regenerating recovery codes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RegenerateRecoveryCodesResponse {
    /// The collection of generated recovery codes.
    #[serde(rename = "recoveryCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_codes: Option<Vec<String>>,
}

/// `Roblox.TwoStepVerification.Api.RenameSecurityKeyRequest`
///
/// Request information needed to rename two step verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RenameSecurityKeyRequest {
    /// A credential ID to be renamed.
    #[serde(rename = "credentialID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<String>,
    /// The new nickname for the credential.
    #[serde(rename = "newNickname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_nickname: Option<String>,
}

crate::api_enum! {
    int
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge. \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    RetractDialogRequestActionType {
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

/// `Roblox.TwoStepVerification.Api.RetractDialogRequest`
///
/// Request parameters for retracting a Cross Device dialog from state ACTIVE to PENDING.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RetractDialogRequest {
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge. \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<RetractDialogRequestActionType>,
    /// The two step verification challenge ID.
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.RetractDialogResponse`
///
/// Result for a successful attempt to retract a Cross Device state from ACTIVE to PENDING.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RetractDialogResponse {}

/// `Roblox.TwoStepVerification.Api.RetryApprovalRequest`
///
/// Request parameters for retrying a Cross Device two step verification approval.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RetryApprovalRequest {
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge. \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<RetractDialogRequestActionType>,
    /// The two step verification challenge ID.
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.RetryApprovalResponse`
///
/// Result for a successful Cross Device approval retry, an empty JSON.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RetryApprovalResponse {}

/// `Roblox.TwoStepVerification.Api.SendCodeRequest`
///
/// Request parameters for sending a two step verification code.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SendCodeRequest {
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge. \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<RetractDialogRequestActionType>,
    /// The two step verification challenge ID.
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
}

crate::api_enum! {
    int
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationMediaType for the method. \['Email' = 0, 'SMS' = 1, 'Authenticator' = 2, 'RecoveryCode' = 3, 'SecurityKey' = 4, 'CrossDevice' = 5, 'Password' = 6, 'Passkey' = 7\]
    UserConfigurationMethodMediaType {
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

/// `Roblox.TwoStepVerification.Api.UserConfigurationMethod`
///
/// A user configuration method option.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserConfigurationMethod {
    /// Whether or not the method is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationMediaType for the method. \['Email' = 0, 'SMS' = 1, 'Authenticator' = 2, 'RecoveryCode' = 3, 'SecurityKey' = 4, 'CrossDevice' = 5, 'Password' = 6, 'Passkey' = 7\]
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<UserConfigurationMethodMediaType>,
    /// The last time this method was updated for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.UserConfiguration`
///
/// The users two step verification configuration.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserConfiguration {
    /// The two step verification methods associated with the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methods: Option<Vec<UserConfigurationMethod>>,
    /// The preferred two step verification method for the user.
    #[serde(rename = "primaryMediaType")]
    #[serde(deserialize_with = "crate::rt::lenient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_media_type: Option<i32>,
}

/// `Roblox.TwoStepVerification.Api.VerifyApprovalRequest`
///
/// Request parameters for verifying a Cross Device two step verification approval. Does not use verification code.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyApprovalRequest {
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge. \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<RetractDialogRequestActionType>,
    /// The two step verification challenge ID.
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.VerifyApprovalResponse`
///
/// Result for a successful Cross Device approval verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyApprovalResponse {
    /// The verification token.
    #[serde(rename = "verificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.VerifyCodeRequest`
///
/// Request parameters for verifying a two step verification code.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyCodeRequest {
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge. \['Unknown' = 0, 'Login' = 1, 'RobuxSpend' = 2, 'ItemTrade' = 3, 'Resale' = 4, 'PasswordReset' = 5, 'RevertAccount' = 6, 'Generic' = 7, 'GenericWithRecoveryCodes' = 8\]
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<RetractDialogRequestActionType>,
    /// The two step verification challenge ID.
    #[serde(rename = "challengeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_id: Option<String>,
    /// The two step verification code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.VerifyCodeResponse`
///
/// Result for a successful verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyCodeResponse {
    /// The verification token.
    #[serde(rename = "verificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.VerifyStartPasskeyResponse`
///
/// Result for a successful verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyStartPasskeyResponse {
    /// The authentication options for the passkey.
    #[serde(rename = "authenticationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_options: Option<String>,
    /// The session of the authentication attempt.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.TwoStepVerification.Api.VerifyStartSecurityKeyResponse`
///
/// Result for a successful verification.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VerifyStartSecurityKeyResponse {
    /// The authentication options for the hardware key.
    #[serde(rename = "authenticationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_options: Option<String>,
    /// The session of the authentication attempt.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// `Roblox.Web.WebAPI.ApiEmptyResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiEmptyResponseModel {}

/// Query, header and form inputs for [`get_metadata`].
#[derive(Debug, Clone, Default)]
pub struct GetMetadataParams {
    /// The user ID.
    pub user_id: Option<i64>,
    /// The active two step verification challenge ID if there is one.
    pub challenge_id: Option<String>,
    /// The Roblox.TwoStepVerification.Client.TwoStepVerificationActionType associated with the challenge.
    pub action_type: Option<i32>,
}

/// Query, header and form inputs for [`get_users_by_user_id_configuration`].
#[derive(Debug, Clone, Default)]
pub struct GetUsersByUserIdConfigurationParams {
    /// The active challenge for the user (as an alternative when the user is unauthenticated).
    pub challenge_id: Option<String>,
    /// The action type the challengeId is associated with.
    pub action_type: Option<i32>,
}

/// `GET /v1/metadata`
///
/// Gets two step verification system metadata.
///
/// The metadata endpoint takes in optional request parameters to output additional context for when the user is unauthenticated but attempting to login with two step verification. When supplied, all three request parameters must be sent and match up.
pub fn get_metadata_request(params: &GetMetadataParams) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v1/metadata".to_string());
    r.query("userId", &params.user_id);
    r.query("challengeId", &params.challenge_id);
    r.query("actionType", &params.action_type);
    Ok(r)
}

/// `GET /v1/metadata`
///
/// Gets two step verification system metadata.
///
/// The metadata endpoint takes in optional request parameters to output additional context for when the user is unauthenticated but attempting to login with two step verification. When supplied, all three request parameters must be sent and match up.
pub async fn get_metadata(client: &Client, params: &GetMetadataParams) -> Result<MetadataResponse> {
    client.send(get_metadata_request(params)?).await?.json()
}

/// `POST /v1/users/{userId}/challenges/authenticator/verify`
///
/// Verifies a two step verification challenge code via authenticator app.
pub fn post_users_by_user_id_challenges_authenticator_verify_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/authenticator/verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/authenticator/verify`
///
/// Verifies a two step verification challenge code via authenticator app.
pub async fn post_users_by_user_id_challenges_authenticator_verify(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_authenticator_verify_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/cross-device/retract`
///
/// Reverts a user's dialog state from ACTIVE to PENDING.
pub fn post_users_by_user_id_challenges_cross_device_retract_request(
    user_id: i64,
    body: &RetractDialogRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/cross-device/retract",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/cross-device/retract`
///
/// Reverts a user's dialog state from ACTIVE to PENDING.
pub async fn post_users_by_user_id_challenges_cross_device_retract(
    client: &Client,
    user_id: i64,
    body: &RetractDialogRequest,
) -> Result<RetractDialogResponse> {
    client
        .send(post_users_by_user_id_challenges_cross_device_retract_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/cross-device/retry`
///
/// Retry a Cross Device two step verification approval.
pub fn post_users_by_user_id_challenges_cross_device_retry_request(
    user_id: i64,
    body: &RetryApprovalRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/cross-device/retry",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/cross-device/retry`
///
/// Retry a Cross Device two step verification approval.
pub async fn post_users_by_user_id_challenges_cross_device_retry(
    client: &Client,
    user_id: i64,
    body: &RetryApprovalRequest,
) -> Result<RetryApprovalResponse> {
    client
        .send(post_users_by_user_id_challenges_cross_device_retry_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/cross-device/verify`
///
/// Verifies a two step verification approval via Cross Device. Cross Device approval does not use a verification code.
pub fn post_users_by_user_id_challenges_cross_device_verify_request(
    user_id: i64,
    body: &VerifyApprovalRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/cross-device/verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/cross-device/verify`
///
/// Verifies a two step verification approval via Cross Device. Cross Device approval does not use a verification code.
pub async fn post_users_by_user_id_challenges_cross_device_verify(
    client: &Client,
    user_id: i64,
    body: &VerifyApprovalRequest,
) -> Result<VerifyApprovalResponse> {
    client
        .send(post_users_by_user_id_challenges_cross_device_verify_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/email/send-code`
///
/// Sends a two step verification challenge code via email.
pub fn post_users_by_user_id_challenges_email_send_code_request(
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/email/send-code",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/email/send-code`
///
/// Sends a two step verification challenge code via email.
pub async fn post_users_by_user_id_challenges_email_send_code(
    client: &Client,
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_challenges_email_send_code_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/email/verify`
///
/// Verifies a two step verification challenge with a code sent via email.
pub fn post_users_by_user_id_challenges_email_verify_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/email/verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/email/verify`
///
/// Verifies a two step verification challenge with a code sent via email.
pub async fn post_users_by_user_id_challenges_email_verify(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_email_verify_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/passkey/verify-finish`
///
/// Validates the assertion data returned by the passkey.
pub fn post_users_by_user_id_challenges_passkey_verify_finish_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/passkey/verify-finish",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/passkey/verify-finish`
///
/// Validates the assertion data returned by the passkey.
pub async fn post_users_by_user_id_challenges_passkey_verify_finish(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_passkey_verify_finish_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/passkey/verify-start`
///
/// Provides a challenge for the passkey to authenticate.
pub fn post_users_by_user_id_challenges_passkey_verify_start_request(
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/passkey/verify-start",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/passkey/verify-start`
///
/// Provides a challenge for the passkey to authenticate.
pub async fn post_users_by_user_id_challenges_passkey_verify_start(
    client: &Client,
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<VerifyStartPasskeyResponse> {
    client
        .send(post_users_by_user_id_challenges_passkey_verify_start_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/password/verify`
///
/// Verifies a two step verification challenge with a password (code).
pub fn post_users_by_user_id_challenges_password_verify_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/password/verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/password/verify`
///
/// Verifies a two step verification challenge with a password (code).
pub async fn post_users_by_user_id_challenges_password_verify(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_password_verify_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/recovery-codes/verify`
///
/// Verifies a two step verification challenge via a recovery code.
///
/// Once a recovery code has been used to verify a challenge it cannot be used again.
pub fn post_users_by_user_id_challenges_recovery_codes_verify_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/recovery-codes/verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/recovery-codes/verify`
///
/// Verifies a two step verification challenge via a recovery code.
///
/// Once a recovery code has been used to verify a challenge it cannot be used again.
pub async fn post_users_by_user_id_challenges_recovery_codes_verify(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_recovery_codes_verify_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/security-key/verify-finish`
///
/// Validates the assertion data returned by the security key.
pub fn post_users_by_user_id_challenges_security_key_verify_finish_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/security-key/verify-finish",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/security-key/verify-finish`
///
/// Validates the assertion data returned by the security key.
pub async fn post_users_by_user_id_challenges_security_key_verify_finish(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_security_key_verify_finish_request(user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/security-key/verify-start`
///
/// Provides a challenge for the security key to authenticate.
pub fn post_users_by_user_id_challenges_security_key_verify_start_request(
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/security-key/verify-start",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/security-key/verify-start`
///
/// Provides a challenge for the security key to authenticate.
pub async fn post_users_by_user_id_challenges_security_key_verify_start(
    client: &Client,
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<VerifyStartSecurityKeyResponse> {
    client
        .send(post_users_by_user_id_challenges_security_key_verify_start_request(user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/sms/send-code`
///
/// Sends a two step verification code via SMS for the specified user.
pub fn post_users_by_user_id_challenges_sms_send_code_request(
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/sms/send-code",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/sms/send-code`
///
/// Sends a two step verification code via SMS for the specified user.
pub async fn post_users_by_user_id_challenges_sms_send_code(
    client: &Client,
    user_id: i64,
    body: &SendCodeRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_challenges_sms_send_code_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/challenges/sms/verify`
///
/// Verifies a two step verification challenge with a code sent via SMS.
pub fn post_users_by_user_id_challenges_sms_verify_request(
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/challenges/sms/verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/challenges/sms/verify`
///
/// Verifies a two step verification challenge with a code sent via SMS.
pub async fn post_users_by_user_id_challenges_sms_verify(
    client: &Client,
    user_id: i64,
    body: &VerifyCodeRequest,
) -> Result<VerifyCodeResponse> {
    client
        .send(post_users_by_user_id_challenges_sms_verify_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/configuration`
///
/// Gets two step verification configuration for the specified user.
pub fn get_users_by_user_id_configuration_request(
    user_id: i64,
    params: &GetUsersByUserIdConfigurationParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/configuration", crate::rt::path_value(&user_id)),
    );
    r.query("challengeId", &params.challenge_id);
    r.query("actionType", &params.action_type);
    Ok(r)
}

/// `GET /v1/users/{userId}/configuration`
///
/// Gets two step verification configuration for the specified user.
pub async fn get_users_by_user_id_configuration(
    client: &Client,
    user_id: i64,
    params: &GetUsersByUserIdConfigurationParams,
) -> Result<UserConfiguration> {
    client
        .send(get_users_by_user_id_configuration_request(user_id, params)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/authenticator/disable`
///
/// Disables two step verification via authenticator for the specified user.
pub fn post_users_by_user_id_configuration_authenticator_disable_request(
    user_id: i64,
    body: &DisableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/authenticator/disable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/authenticator/disable`
///
/// Disables two step verification via authenticator for the specified user.
pub async fn post_users_by_user_id_configuration_authenticator_disable(
    client: &Client,
    user_id: i64,
    body: &DisableTwoStepVerificationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_authenticator_disable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/authenticator/enable`
///
/// Initiates enabling authenticator-based two step verification for the specified user.
pub fn post_users_by_user_id_configuration_authenticator_enable_request(
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/authenticator/enable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/authenticator/enable`
///
/// Initiates enabling authenticator-based two step verification for the specified user.
pub async fn post_users_by_user_id_configuration_authenticator_enable(
    client: &Client,
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<EnableAuthenticatorResponse> {
    client
        .send(post_users_by_user_id_configuration_authenticator_enable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/authenticator/enable-verify`
///
/// Finishes enabling authenticator-based two step verification for the specified user.
///
/// Enabling authenticator-based two step verification requires two parts to help ensure the user has properly stored the authenticator key in their authenticator app.
pub fn post_users_by_user_id_configuration_authenticator_enable_verify_request(
    user_id: i64,
    body: &EnableVerifyAuthenticatorRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/authenticator/enable-verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/authenticator/enable-verify`
///
/// Finishes enabling authenticator-based two step verification for the specified user.
///
/// Enabling authenticator-based two step verification requires two parts to help ensure the user has properly stored the authenticator key in their authenticator app.
pub async fn post_users_by_user_id_configuration_authenticator_enable_verify(
    client: &Client,
    user_id: i64,
    body: &EnableVerifyAuthenticatorRequest,
) -> Result<EnableVerifyAuthenticatorResponse> {
    client
        .send(post_users_by_user_id_configuration_authenticator_enable_verify_request(user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/email/disable`
///
/// Disables two step verification via email for the specified user.
pub fn post_users_by_user_id_configuration_email_disable_request(
    user_id: i64,
    body: &DisableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/email/disable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/email/disable`
///
/// Disables two step verification via email for the specified user.
pub async fn post_users_by_user_id_configuration_email_disable(
    client: &Client,
    user_id: i64,
    body: &DisableTwoStepVerificationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_email_disable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/email/enable`
///
/// Enables two step verification via email for the specified user.
pub fn post_users_by_user_id_configuration_email_enable_request(
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/email/enable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/email/enable`
///
/// Enables two step verification via email for the specified user.
pub async fn post_users_by_user_id_configuration_email_enable(
    client: &Client,
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_email_enable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/security-key/disable`
///
/// Disables a batch of credentials for the specified user.
pub fn post_users_by_user_id_configuration_security_key_disable_request(
    user_id: i64,
    body: &DisableSecurityKeyRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/security-key/disable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/security-key/disable`
///
/// Disables a batch of credentials for the specified user.
pub async fn post_users_by_user_id_configuration_security_key_disable(
    client: &Client,
    user_id: i64,
    body: &DisableSecurityKeyRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_security_key_disable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/security-key/enable`
///
/// Initiates security key registration by providing credential creation options.
pub fn post_users_by_user_id_configuration_security_key_enable_request(
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/security-key/enable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/security-key/enable`
///
/// Initiates security key registration by providing credential creation options.
pub async fn post_users_by_user_id_configuration_security_key_enable(
    client: &Client,
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<EnableSecurityKeyResponse> {
    client
        .send(post_users_by_user_id_configuration_security_key_enable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/security-key/enable-verify`
///
/// Finishes security key registration and stores credential. Enables security key as a 2sv media type if it is a user's first key.
pub fn post_users_by_user_id_configuration_security_key_enable_verify_request(
    user_id: i64,
    body: &EnableVerifySecurityKeyRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/security-key/enable-verify",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/security-key/enable-verify`
///
/// Finishes security key registration and stores credential. Enables security key as a 2sv media type if it is a user's first key.
pub async fn post_users_by_user_id_configuration_security_key_enable_verify(
    client: &Client,
    user_id: i64,
    body: &EnableVerifySecurityKeyRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_security_key_enable_verify_request(user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/security-key/list`
///
/// List a user's registered security keys.
pub fn post_users_by_user_id_configuration_security_key_list_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/security-key/list",
            crate::rt::path_value(&user_id)
        ),
    );
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/security-key/list`
///
/// List a user's registered security keys.
pub async fn post_users_by_user_id_configuration_security_key_list(
    client: &Client,
    user_id: i64,
) -> Result<ListSecurityKeyResponse> {
    client
        .send(post_users_by_user_id_configuration_security_key_list_request(
            user_id,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/security-key/rename`
///
/// Rename a user's security key.
pub fn post_users_by_user_id_configuration_security_key_rename_request(
    user_id: i64,
    body: &RenameSecurityKeyRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/security-key/rename",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/security-key/rename`
///
/// Rename a user's security key.
pub async fn post_users_by_user_id_configuration_security_key_rename(
    client: &Client,
    user_id: i64,
    body: &RenameSecurityKeyRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_security_key_rename_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/sms/disable`
///
/// Disables two step verification via SMS for the specified user.
pub fn post_users_by_user_id_configuration_sms_disable_request(
    user_id: i64,
    body: &DisableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/sms/disable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/sms/disable`
///
/// Disables two step verification via SMS for the specified user.
pub async fn post_users_by_user_id_configuration_sms_disable(
    client: &Client,
    user_id: i64,
    body: &DisableTwoStepVerificationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_sms_disable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/configuration/sms/enable`
///
/// Enables two step verification via SMS for the specified user.
pub fn post_users_by_user_id_configuration_sms_enable_request(
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/configuration/sms/enable",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/configuration/sms/enable`
///
/// Enables two step verification via SMS for the specified user.
pub async fn post_users_by_user_id_configuration_sms_enable(
    client: &Client,
    user_id: i64,
    body: &EnableTwoStepVerificationRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_configuration_sms_enable_request(
            user_id, body,
        )?)
        .await?
        .json()
}

/// `GET /v1/users/{userId}/recovery-codes`
///
/// Gets the current status of recovery codes for a user.
pub fn get_users_by_user_id_recovery_codes_request(user_id: i64) -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        format!("/v1/users/{}/recovery-codes", crate::rt::path_value(&user_id)),
    );
    Ok(r)
}

/// `GET /v1/users/{userId}/recovery-codes`
///
/// Gets the current status of recovery codes for a user.
pub async fn get_users_by_user_id_recovery_codes(
    client: &Client,
    user_id: i64,
) -> Result<RecoveryCodesStatusResponse> {
    client
        .send(get_users_by_user_id_recovery_codes_request(user_id)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/recovery-codes/clear`
///
/// Clears any existing recovery codes for the user.
///
/// Clearing recovery codes voids any recovery codes previously generated for the user. New recovery codes will have to be generated to pass two step verification via recovery code.
pub fn post_users_by_user_id_recovery_codes_clear_request(
    user_id: i64,
    body: &ClearRecoveryCodesRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/recovery-codes/clear",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/recovery-codes/clear`
///
/// Clears any existing recovery codes for the user.
///
/// Clearing recovery codes voids any recovery codes previously generated for the user. New recovery codes will have to be generated to pass two step verification via recovery code.
pub async fn post_users_by_user_id_recovery_codes_clear(
    client: &Client,
    user_id: i64,
    body: &ClearRecoveryCodesRequest,
) -> Result<ApiEmptyResponseModel> {
    client
        .send(post_users_by_user_id_recovery_codes_clear_request(user_id, body)?)
        .await?
        .json()
}

/// `POST /v1/users/{userId}/recovery-codes/regenerate`
///
/// Clears any existing recovery codes and generates a new batch of recovery codes.
///
/// Two step verification recovery codes do not enforce that two step verification must be passed when logging in. At least one two step verification media type must be enabled to trigger the two step verification flow. Recovery codes are intended to be used to pass two step verification when the enabled media type is unavailable. Recovery codes generated by this endpoint do not have an expiration. Once a recovery code generated by this endpoint has been used it cannot be used again.
pub fn post_users_by_user_id_recovery_codes_regenerate_request(
    user_id: i64,
    body: &RegenerateRecoveryCodesRequest,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        format!(
            "/v1/users/{}/recovery-codes/regenerate",
            crate::rt::path_value(&user_id)
        ),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v1/users/{userId}/recovery-codes/regenerate`
///
/// Clears any existing recovery codes and generates a new batch of recovery codes.
///
/// Two step verification recovery codes do not enforce that two step verification must be passed when logging in. At least one two step verification media type must be enabled to trigger the two step verification flow. Recovery codes are intended to be used to pass two step verification when the enabled media type is unavailable. Recovery codes generated by this endpoint do not have an expiration. Once a recovery code generated by this endpoint has been used it cannot be used again.
pub async fn post_users_by_user_id_recovery_codes_regenerate(
    client: &Client,
    user_id: i64,
    body: &RegenerateRecoveryCodesRequest,
) -> Result<RegenerateRecoveryCodesResponse> {
    client
        .send(post_users_by_user_id_recovery_codes_regenerate_request(
            user_id, body,
        )?)
        .await?
        .json()
}
