use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    code: u16,
    message: String,
}

// --------- submit-listens

#[derive(Debug, Deserialize)]
pub struct SubmitListensResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,
}

// --------- validate-token

#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub valid: bool,
    pub user_name: Option<String>,
}

// --------- delete-listen

#[derive(Debug, Deserialize)]
pub struct DeleteListenResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,
}

// --------- users/{user_list}/recent-listens

#[derive(Debug, Deserialize)]
pub struct UsersRecentListensResponse {
    pub payload: UsersRecentListensPayload,
}

#[derive(Debug, Deserialize)]
pub struct UsersRecentListensPayload {
    pub count: u64,
    pub listens: Vec<UsersRecentListensListen>,
    pub user_list: String,
}

#[derive(Debug, Deserialize)]
pub struct UsersRecentListensListen {
    pub user_name: String,
    pub inserted_at: String,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UsersRecentListensTrackMetadata,
}

#[derive(Debug, Deserialize)]
pub struct UsersRecentListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// --------- user/{user_name}/listen-count

#[derive(Debug, Deserialize)]
pub struct UserListenCountResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub count: u64,
}

// -------- user/{user_name}/playing-now

#[derive(Debug, Deserialize)]
pub struct UserPlayingNowResponse {
    pub payload: UserPlayingNowPayload,
}

#[derive(Debug, Deserialize)]
pub struct UserPlayingNowPayload {
    pub count: u8,
    pub user_id: String,
    pub listens: Vec<UserPlayingNowListen>,
}

#[derive(Debug, Deserialize)]
pub struct UserPlayingNowListen {
    pub user_name: String,
    pub inserted_at: String,
    pub recording_msid: String,
    pub track_metadata: UserPlayingNowTrackMetadata,
}

#[derive(Debug, Deserialize)]
pub struct UserPlayingNowTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// -------- user/{user_name}/listens

#[derive(Debug, Deserialize)]
pub struct UserListensResponse {
    pub payload: UserListensPayload,
}

#[derive(Debug, Deserialize)]
pub struct UserListensPayload {
    pub count: u64,
    pub latest_listen_ts: i64,
    pub user_id: String,
    pub listens: Vec<UserListensListen>,
}

#[derive(Debug, Deserialize)]
pub struct UserListensListen {
    pub user_name: String,
    pub inserted_at: String,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UserListensTrackMetadata,
}

#[derive(Debug, Deserialize)]
pub struct UserListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// --------- latest-import (GET)

#[derive(Debug, Deserialize)]
pub struct GetLatestImportResponse {
    pub latest_import: i64,
    pub musicbrainz_id: String,
}

// --------- latest-import (POST)

#[derive(Debug, Deserialize)]
pub struct UpdateLatestImportResponse {
    pub status: String,
}

// --------- status/get-dump-info

#[derive(Debug, Deserialize)]
pub struct StatusGetDumpInfoResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub id: i64,
    pub timestamp: String,
}
