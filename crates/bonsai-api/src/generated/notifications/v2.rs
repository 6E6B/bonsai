//! `notifications.roblox.com` v2 — generated from `vendor/roblox-api-tracker/notifications/v2.json`.
//!
//! Do not edit. Change the spec or `bonsai-codegen` and run `cargo xtask codegen`.

#![allow(deprecated, unused_imports, clippy::all)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rt::{Client, FilePart, Method, Request, Result};

/// Host for every operation in this module.
pub const HOST: &str = "notifications.roblox.com";

/// `Roblox.Api.Notifications.Models.ChromeManifestModel`
///
/// Chrome Manifest to link GCM project to Chrome browser
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChromeManifestModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcm_sender_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// `Roblox.Api.Notifications.Models.GameUpdateNotificationModel`
///
/// Model for a game update message to be displayed in notification stream.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GameUpdateNotificationModel {
    /// Content of the update message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Creation date of the update message.
    #[serde(rename = "createdOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    /// Key of Creation date ticks (dynamo table sort key).
    #[serde(rename = "createdOnKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on_key: Option<String>,
    /// ID of the game.
    #[serde(rename = "rootPlaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_place_id: Option<i64>,
    /// ID of the game.
    #[serde(rename = "universeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<i64>,
    /// Name of the game.
    #[serde(rename = "universeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universe_name: Option<String>,
}

crate::api_enum! {
    str
    /// Platform for the corresponding notification \['ChromeOnDesktop' = 0, 'AndroidNative' = 1, 'FirefoxOnDesktop' = 2, 'IOSNative' = 3, 'AndroidAmazon' = 4, 'IOSTencent' = 5, 'AndroidTencentService' = 6, 'IOSPushKit' = 7\]
    UserPushDestinationPlatform {
    ChromeOnDesktop = 0 => "ChromeOnDesktop",
    AndroidNative = 1 => "AndroidNative",
    FirefoxOnDesktop = 2 => "FirefoxOnDesktop",
    IosNative = 3 => "IOSNative",
    AndroidAmazon = 4 => "AndroidAmazon",
    IosTencent = 5 => "IOSTencent",
    AndroidTencentService = 6 => "AndroidTencentService",
    IosPushKit = 7 => "IOSPushKit",
    }
}

/// `Roblox.Api.Notifications.Models.NotificationUser`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationUser {
    /// Name of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Id of the user
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// `Roblox.Api.Notifications.Models.UserPushDestination`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPushDestination {
    /// application for the corresponding notification Example : "Roblox"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    /// Name of the destination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Token for the corresponding notification
    #[serde(rename = "notificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_token: Option<String>,
    /// Platform for the corresponding notification \['ChromeOnDesktop' = 0, 'AndroidNative' = 1, 'FirefoxOnDesktop' = 2, 'IOSNative' = 3, 'AndroidAmazon' = 4, 'IOSTencent' = 5, 'AndroidTencentService' = 6, 'IOSPushKit' = 7\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<UserPushDestinationPlatform>,
    /// Is Update notification feature supported
    #[serde(rename = "supportsUpdateNotifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_update_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<NotificationUser>,
    /// Destination Id for the push notification
    #[serde(rename = "userPushNotificationDestinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_push_notification_destination_id: Option<i64>,
}

/// `Roblox.Api.Notifications.Models.GetCurrentPushDestinationResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetCurrentPushDestinationResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<UserPushDestination>,
    /// Message for the success response
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Api.Notifications.Models.PushNotificationClientMetadata`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PushNotificationClientMetadata {
    /// Details corresponding to the notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
    /// Is fallback delivered for the notification metadata
    #[serde(rename = "fallbackDelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_delivered: Option<bool>,
    /// Id for the push client notification
    #[serde(rename = "notificationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    /// Type of the push client notification
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// `Roblox.Api.Notifications.Models.GetMetadataResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetMetadataResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PushNotificationClientMetadata>,
    /// Message for the success response
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Api.Notifications.Models.GetPushDestinationsResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetPushDestinationsResponseModel {
    /// List of destinations for the user push notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<UserPushDestination>>,
    /// Message for the success response
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

crate::api_enum! {
    str
    /// Type of the notification source \['Test' = 0, 'FriendRequestReceived' = 1, 'FriendRequestAccepted' = 2, 'PartyInviteReceived' = 3, 'PartyMemberJoined' = 4, 'ChatNewMessage' = 5, 'PrivateMessageReceived' = 6, 'UserAddedToPrivateServerWhiteList' = 7, 'ConversationUniverseChanged' = 8, 'TeamCreateInvite' = 9, 'GameUpdate' = 10, 'DeveloperMetricsAvailable' = 11, 'GroupJoinRequestAccepted' = 12, 'Sendr' = 13, 'ExperienceInvitation' = 14\]
    NotificationStreamEntriesModelNotificationSourceType {
    Test = 0 => "Test",
    FriendRequestReceived = 1 => "FriendRequestReceived",
    FriendRequestAccepted = 2 => "FriendRequestAccepted",
    PartyInviteReceived = 3 => "PartyInviteReceived",
    PartyMemberJoined = 4 => "PartyMemberJoined",
    ChatNewMessage = 5 => "ChatNewMessage",
    PrivateMessageReceived = 6 => "PrivateMessageReceived",
    UserAddedToPrivateServerWhiteList = 7 => "UserAddedToPrivateServerWhiteList",
    ConversationUniverseChanged = 8 => "ConversationUniverseChanged",
    TeamCreateInvite = 9 => "TeamCreateInvite",
    GameUpdate = 10 => "GameUpdate",
    DeveloperMetricsAvailable = 11 => "DeveloperMetricsAvailable",
    GroupJoinRequestAccepted = 12 => "GroupJoinRequestAccepted",
    Sendr = 13 => "Sendr",
    ExperienceInvitation = 14 => "ExperienceInvitation",
    }
}

/// `Roblox.Api.Notifications.Models.NotificationStreamEntriesModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationStreamEntriesModel {
    /// Content object for sendr notification stream entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    /// Count of events corresponding to the group of notification stream entry
    #[serde(rename = "eventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    /// Datetime when the notification stream entry event had occured
    #[serde(rename = "eventDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    /// Id of the notification stream entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Has the user interacted with the notification stream entry
    #[serde(rename = "isInteracted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_interacted: Option<bool>,
    /// List of metadata objects showing more details related to the notification stream entry
    #[serde(rename = "metadataCollection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_collection: Option<Vec<serde_json::Value>>,
    /// Type of the notification source \['Test' = 0, 'FriendRequestReceived' = 1, 'FriendRequestAccepted' = 2, 'PartyInviteReceived' = 3, 'PartyMemberJoined' = 4, 'ChatNewMessage' = 5, 'PrivateMessageReceived' = 6, 'UserAddedToPrivateServerWhiteList' = 7, 'ConversationUniverseChanged' = 8, 'TeamCreateInvite' = 9, 'GameUpdate' = 10, 'DeveloperMetricsAvailable' = 11, 'GroupJoinRequestAccepted' = 12, 'Sendr' = 13, 'ExperienceInvitation' = 14\]
    #[serde(rename = "notificationSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_source_type: Option<NotificationStreamEntriesModelNotificationSourceType>,
    /// Relative timestamp for sendr notification stream entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// `Roblox.Api.Notifications.Models.PushNotificationRegistration`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PushNotificationRegistration {
    /// Notification application Example : "Roblox"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    /// Name of the push Notification destination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Notification token
    #[serde(rename = "notificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_token: Option<String>,
    /// Notification platform \['ChromeOnDesktop' = 0, 'AndroidNative' = 1, 'FirefoxOnDesktop' = 2, 'IOSNative' = 3, 'AndroidAmazon' = 4, 'IOSTencent' = 5, 'AndroidTencentService' = 6, 'IOSPushKit' = 7\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<UserPushDestinationPlatform>,
    /// Id of the push notification destination
    #[serde(rename = "userPushNotificationDestinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_push_notification_destination_id: Option<i64>,
}

/// `Roblox.Api.Notifications.Models.RegisterAndroidRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RegisterAndroidRequestModel {
    /// Is this call authorized for user
    #[serde(rename = "authorizeForUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize_for_user: Option<bool>,
    /// Name of the requesting device
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Token for notification
    #[serde(rename = "notificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_token: Option<String>,
    /// Old notification token
    #[serde(rename = "oldNotificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_notification_token: Option<String>,
}

/// `Roblox.Api.Notifications.Models.RegisterIOSNativeRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RegisterIosNativeRequestModel {
    /// Is call authorized for user
    #[serde(rename = "authorizeForUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize_for_user: Option<bool>,
    /// Identifier string for the destination
    #[serde(rename = "destinationIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_identifier: Option<String>,
    /// Name of the requesting device
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// True when the client granted iOS provisional authorization (silent, quiet-only) rather than full authorization. Absent/null is treated as not provisional.
    #[serde(rename = "isProvisional")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_provisional: Option<bool>,
    /// Token for notification
    #[serde(rename = "notificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_token: Option<String>,
    /// Old notification token
    #[serde(rename = "oldNotificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_notification_token: Option<String>,
}

/// `Roblox.Api.Notifications.Models.RegisterIOSPushKitRequestModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RegisterIosPushKitRequestModel {
    /// Is call authorized for user
    #[serde(rename = "authorizeForUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize_for_user: Option<bool>,
    /// Identifier string for the destination
    #[serde(rename = "destinationIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_identifier: Option<String>,
    /// Name of the requesting device
    #[serde(rename = "deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Token for notification
    #[serde(rename = "notificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_token: Option<String>,
    /// Old notification token
    #[serde(rename = "oldNotificationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_notification_token: Option<String>,
}

/// `Roblox.Api.Notifications.Models.RegistrationResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RegistrationResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<PushNotificationRegistration>,
    /// Message for the success response
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Api.Notifications.Models.ResponseModels.NotificationStreamMetadataResponse`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationStreamMetadataResponse {
    #[serde(rename = "bannerDismissTimeSpan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_dismiss_time_span: Option<i32>,
    #[serde(rename = "canLaunchGameFromGameUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_launch_game_from_game_update: Option<bool>,
    #[serde(rename = "signalRDisconnectionResponseInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_r_disconnection_response_in_milliseconds: Option<i64>,
}

/// `Roblox.Api.Notifications.Models.SuccessResponseModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SuccessResponseModel {
    /// Message for the success response
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

/// `Roblox.Api.Notifications.Models.UnreadStreamNotificationsModel`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UnreadStreamNotificationsModel {
    /// Message for the success response
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// Count of unread notifications in the stream
    #[serde(rename = "unreadNotifications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_notifications: Option<i64>,
}

/// Query, header and form inputs for [`get_push_notifications_metadata`].
#[derive(Debug, Clone, Default)]
pub struct GetPushNotificationsMetadataParams {
    /// Token for the notification
    pub notification_token: String,
    /// Id of the specified notification
    pub notification_id: String,
}

/// Query, header and form inputs for [`get_stream_notifications_get_latest_game_updates`].
#[derive(Debug, Clone, Default)]
pub struct GetStreamNotificationsGetLatestGameUpdatesParams {
    /// List of universe IDs
    pub universe_ids: Vec<i64>,
    /// For retrieving only updates that created after a time point.
    pub since_date_time: Option<String>,
}

/// Query, header and form inputs for [`get_stream_notifications_get_recent`].
#[derive(Debug, Clone, Default)]
pub struct GetStreamNotificationsGetRecentParams {
    /// Index to start the entries from. (Optional : Defaults to 0 which means the most recent entry)
    pub start_index: Option<i32>,
    /// Number of entries to be returned. (Optional : Defaults to 10 entries)
    pub max_rows: Option<i32>,
}

/// `GET /v2/push-notifications/chrome-manifest`
///
/// Get Chrome Manifest to link GCM project to Chrome Browser
pub fn get_push_notifications_chrome_manifest_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v2/push-notifications/chrome-manifest".to_string(),
    );
    Ok(r)
}

/// `GET /v2/push-notifications/chrome-manifest`
///
/// Get Chrome Manifest to link GCM project to Chrome Browser
pub async fn get_push_notifications_chrome_manifest(client: &Client) -> Result<ChromeManifestModel> {
    client
        .send(get_push_notifications_chrome_manifest_request()?)
        .await?
        .json()
}

/// `POST /v2/push-notifications/deregister-all-devices`
///
/// De-register all devices to disable push notifications
pub fn post_push_notifications_deregister_all_devices_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/v2/push-notifications/deregister-all-devices".to_string(),
    );
    Ok(r)
}

/// `POST /v2/push-notifications/deregister-all-devices`
///
/// De-register all devices to disable push notifications
pub async fn post_push_notifications_deregister_all_devices(client: &Client) -> Result<SuccessResponseModel> {
    client
        .send(post_push_notifications_deregister_all_devices_request()?)
        .await?
        .json()
}

/// `POST /v2/push-notifications/deregister-current-device`
///
/// De-register current device to disable push notifications
pub fn post_push_notifications_deregister_current_device_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/v2/push-notifications/deregister-current-device".to_string(),
    );
    Ok(r)
}

/// `POST /v2/push-notifications/deregister-current-device`
///
/// De-register current device to disable push notifications
pub async fn post_push_notifications_deregister_current_device(
    client: &Client,
) -> Result<SuccessResponseModel> {
    client
        .send(post_push_notifications_deregister_current_device_request()?)
        .await?
        .json()
}

/// `POST /v2/push-notifications/deregister-current-device-ios-pushkit`
///
/// De-register current device to disable pushkit notifications
pub fn post_push_notifications_deregister_current_device_ios_pushkit_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/v2/push-notifications/deregister-current-device-ios-pushkit".to_string(),
    );
    Ok(r)
}

/// `POST /v2/push-notifications/deregister-current-device-ios-pushkit`
///
/// De-register current device to disable pushkit notifications
pub async fn post_push_notifications_deregister_current_device_ios_pushkit(
    client: &Client,
) -> Result<SuccessResponseModel> {
    client
        .send(post_push_notifications_deregister_current_device_ios_pushkit_request()?)
        .await?
        .json()
}

/// `GET /v2/push-notifications/get-current-device-destination`
///
/// Gets the current device destination
pub fn get_push_notifications_get_current_device_destination_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v2/push-notifications/get-current-device-destination".to_string(),
    );
    Ok(r)
}

/// `GET /v2/push-notifications/get-current-device-destination`
///
/// Gets the current device destination
pub async fn get_push_notifications_get_current_device_destination(
    client: &Client,
) -> Result<GetCurrentPushDestinationResponseModel> {
    client
        .send(get_push_notifications_get_current_device_destination_request()?)
        .await?
        .json()
}

/// `GET /v2/push-notifications/get-destinations`
///
/// Gets valid destinations associated with the signed user
pub fn get_push_notifications_get_destinations_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v2/push-notifications/get-destinations".to_string(),
    );
    Ok(r)
}

/// `GET /v2/push-notifications/get-destinations`
///
/// Gets valid destinations associated with the signed user
pub async fn get_push_notifications_get_destinations(
    client: &Client,
) -> Result<GetPushDestinationsResponseModel> {
    client
        .send(get_push_notifications_get_destinations_request()?)
        .await?
        .json()
}

/// `GET /v2/push-notifications/metadata`
///
/// Gets the corresponding metadata for the specified notification
pub fn get_push_notifications_metadata_request(
    params: &GetPushNotificationsMetadataParams,
) -> Result<Request> {
    let mut r = Request::new(Method::GET, HOST, "/v2/push-notifications/metadata".to_string());
    r.query("notificationToken", &params.notification_token);
    r.query("notificationId", &params.notification_id);
    Ok(r)
}

/// `GET /v2/push-notifications/metadata`
///
/// Gets the corresponding metadata for the specified notification
pub async fn get_push_notifications_metadata(
    client: &Client,
    params: &GetPushNotificationsMetadataParams,
) -> Result<GetMetadataResponseModel> {
    client
        .send(get_push_notifications_metadata_request(params)?)
        .await?
        .json()
}

/// `POST /v2/push-notifications/register-android-native`
///
/// Register Android Native for push notifications
pub fn post_push_notifications_register_android_native_request(
    body: &RegisterAndroidRequestModel,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/push-notifications/register-android-native".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/push-notifications/register-android-native`
///
/// Register Android Native for push notifications
pub async fn post_push_notifications_register_android_native(
    client: &Client,
    body: &RegisterAndroidRequestModel,
) -> Result<RegistrationResponseModel> {
    client
        .send(post_push_notifications_register_android_native_request(body)?)
        .await?
        .json()
}

/// `POST /v2/push-notifications/register-ios-native`
///
/// Registers IOS device for push notifications
pub fn post_push_notifications_register_ios_native_request(
    body: &RegisterIosNativeRequestModel,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/push-notifications/register-ios-native".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/push-notifications/register-ios-native`
///
/// Registers IOS device for push notifications
pub async fn post_push_notifications_register_ios_native(
    client: &Client,
    body: &RegisterIosNativeRequestModel,
) -> Result<RegistrationResponseModel> {
    client
        .send(post_push_notifications_register_ios_native_request(body)?)
        .await?
        .json()
}

/// `POST /v2/push-notifications/register-ios-pushkit`
///
/// Registers IOS device for pushkit notifications
pub fn post_push_notifications_register_ios_pushkit_request(
    body: &RegisterIosPushKitRequestModel,
) -> Result<Request> {
    let mut r = Request::new(
        Method::POST,
        HOST,
        "/v2/push-notifications/register-ios-pushkit".to_string(),
    );
    r.json(body)?;
    Ok(r)
}

/// `POST /v2/push-notifications/register-ios-pushkit`
///
/// Registers IOS device for pushkit notifications
pub async fn post_push_notifications_register_ios_pushkit(
    client: &Client,
    body: &RegisterIosPushKitRequestModel,
) -> Result<RegistrationResponseModel> {
    client
        .send(post_push_notifications_register_ios_pushkit_request(body)?)
        .await?
        .json()
}

/// `POST /v2/stream-notifications/clear-unread`
///
/// Clears the unread Notification stream count
pub fn post_stream_notifications_clear_unread_request() -> Result<Request> {
    let r = Request::new(
        Method::POST,
        HOST,
        "/v2/stream-notifications/clear-unread".to_string(),
    );
    Ok(r)
}

/// `POST /v2/stream-notifications/clear-unread`
///
/// Clears the unread Notification stream count
pub async fn post_stream_notifications_clear_unread(client: &Client) -> Result<SuccessResponseModel> {
    client
        .send(post_stream_notifications_clear_unread_request()?)
        .await?
        .json()
}

/// `GET /v2/stream-notifications/get-latest-game-updates`
///
/// Get the latest non aggregated Game Updates sent to the logged in user
pub fn get_stream_notifications_get_latest_game_updates_request(
    params: &GetStreamNotificationsGetLatestGameUpdatesParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v2/stream-notifications/get-latest-game-updates".to_string(),
    );
    r.query("universeIds", &params.universe_ids);
    r.query("sinceDateTime", &params.since_date_time);
    Ok(r)
}

/// `GET /v2/stream-notifications/get-latest-game-updates`
///
/// Get the latest non aggregated Game Updates sent to the logged in user
pub async fn get_stream_notifications_get_latest_game_updates(
    client: &Client,
    params: &GetStreamNotificationsGetLatestGameUpdatesParams,
) -> Result<Vec<GameUpdateNotificationModel>> {
    client
        .send(get_stream_notifications_get_latest_game_updates_request(params)?)
        .await?
        .json()
}

/// `GET /v2/stream-notifications/get-recent`
///
/// Gets the recent entries from the notification stream
pub fn get_stream_notifications_get_recent_request(
    params: &GetStreamNotificationsGetRecentParams,
) -> Result<Request> {
    let mut r = Request::new(
        Method::GET,
        HOST,
        "/v2/stream-notifications/get-recent".to_string(),
    );
    r.query("startIndex", &params.start_index);
    r.query("maxRows", &params.max_rows);
    Ok(r)
}

/// `GET /v2/stream-notifications/get-recent`
///
/// Gets the recent entries from the notification stream
pub async fn get_stream_notifications_get_recent(
    client: &Client,
    params: &GetStreamNotificationsGetRecentParams,
) -> Result<Vec<NotificationStreamEntriesModel>> {
    client
        .send(get_stream_notifications_get_recent_request(params)?)
        .await?
        .json()
}

/// `GET /v2/stream-notifications/metadata`
///
/// Get Notification Stream metadata.
pub fn get_stream_notifications_metadata_request() -> Result<Request> {
    let r = Request::new(Method::GET, HOST, "/v2/stream-notifications/metadata".to_string());
    Ok(r)
}

/// `GET /v2/stream-notifications/metadata`
///
/// Get Notification Stream metadata.
pub async fn get_stream_notifications_metadata(
    client: &Client,
) -> Result<NotificationStreamMetadataResponse> {
    client
        .send(get_stream_notifications_metadata_request()?)
        .await?
        .json()
}

/// `GET /v2/stream-notifications/unread-count`
///
/// Gets the count of unread Notification stream entries
pub fn get_stream_notifications_unread_count_request() -> Result<Request> {
    let r = Request::new(
        Method::GET,
        HOST,
        "/v2/stream-notifications/unread-count".to_string(),
    );
    Ok(r)
}

/// `GET /v2/stream-notifications/unread-count`
///
/// Gets the count of unread Notification stream entries
pub async fn get_stream_notifications_unread_count(
    client: &Client,
) -> Result<UnreadStreamNotificationsModel> {
    client
        .send(get_stream_notifications_unread_count_request()?)
        .await?
        .json()
}
