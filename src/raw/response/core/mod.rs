use std::collections::HashMap;

use serde::Serialize;

use super::response_type;
use crate::raw::response::Deserialize;

// --------- GET /1/search/users/
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-search-users-

// TODO

// --------- POST /1/submit-listens
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#post--1-submit-listens

response_type! {
    /// Response type for [`Client::submit_listens`](super::Client::submit_listens).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct SubmitListensResponse {
        pub status: String,
    }
}

// --------- GET /1/user/(user_name)/listens
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-listens

response_type! {
    /// Response type for [`Client::user_listens`](super::Client::user_listens).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct UserListensResponse {
        pub payload: UserListensPayload,
    }
}

/// Type of the [`UserListensResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensPayload {
    pub count: u64,
    pub latest_listen_ts: i64,
    pub oldest_listen_ts: i64,
    pub user_id: String,
    pub listens: Vec<UserListensListen>,
}

/// Type of the [`UserListensPayload::listens`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensListen {
    pub user_name: String,
    pub inserted_at: i64,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UserListensTrackMetadata,
}

/// Type of the [`UserListensListen::track_metadata`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
    pub mbid_mapping: Option<UserListensMBIDMapping>,
}

/// Type of the [`UserListensTrackMetadata::mbid_mapping`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensMBIDMapping {
    pub artist_mbids: Option<Vec<String>>,
    pub artists: Option<Vec<UserListensMappingArtist>>,
    pub recording_mbid: String,
    pub recording_name: Option<String>,
    pub caa_id: Option<u64>,
    pub caa_release_mbid: Option<String>,
    pub release_mbid: Option<String>,
}

/// Type of the [`UserListensMBIDMapping::artists`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListensMappingArtist {
    pub artist_mbid: String,
    pub artist_credit_name: String,
    pub join_phrase: String,
}

// --------- GET /1/user/(user_name)/listen-count
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-listen-count

response_type! {
    /// Response type for [`Client::user_listen_count`](super::Client::user_listen_count).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct UserListenCountResponse {
        pub payload: UserListenCountPayload,
    }
}

/// Type of the [`UserListenCountResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserListenCountPayload {
    pub count: u64,
}

// --------- GET /1/user/(user_name)/playing-now
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-playing-now

response_type! {
    /// Response type for [`Client::user_playing_now`](super::Client::user_playing_now).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct UserPlayingNowResponse {
        pub payload: UserPlayingNowPayload,
    }
}

/// Type of the [`UserPlayingNowResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserPlayingNowPayload {
    pub count: u8,
    pub user_id: String,
    pub listens: Vec<UserPlayingNowListen>,
    pub playing_now: bool,
}

/// Type of the [`UserPlayingNowPayload::listens`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserPlayingNowListen {
    pub track_metadata: UserPlayingNowTrackMetadata,
    pub playing_now: bool,
}

/// Type of the [`UserPlayingNowListen::track_metadata`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserPlayingNowTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// ---------  GET /1/user/(user_name)/similar-users
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-similar-users

response_type! {
    /// Response type for [`Client::user_similar_users`](super::Client::user_similar_users).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
    pub struct UserSimilarUsersResponse {
        pub payload: Vec<UserSimilarUsersPayload>,
    }
}

/// Type of the [`UserSimilarUsersResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserSimilarUsersPayload {
    pub user_name: String,
    pub similarity: f64,
}

// --------- GET /1/user/(user_name)/similar-to/(other_user_name)
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-similar-to-(other_user_name)

response_type! {
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserSimilarToResponse {
        pub user_name: String,
        pub similarity: f64,
    }
}

// --------- GET /1/validate-token
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-validate-token

response_type! {
    /// Response type for [`Client::validate_token`](super::Client::validate_token).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct ValidateTokenResponse {
        pub code: u16,
        pub message: String,

        pub valid: bool,
        pub user_name: Option<String>,
    }
}

// --------- POST /1/delete-listen
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#post--1-delete-listen

response_type! {
    /// Response type for [`Client::delete_listen`](super::Client::delete_listen).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct DeleteListenResponse {
        pub status: String,
    }
}

// --------- GET /1/user/(playlist_user_name)/playlists/recommendations
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(playlist_user_name)-playlists-recommendations

// TODO

// --------- GET /1/user/(user_name)/services
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-services

// TODO

// --------- GET /1/lb-radio/tags
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-lb-radio-tags

// TODO

// --------- GET /1/lb-radio/artist/(seed_artist_mbid)
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-lb-radio-artist-(seed_artist_mbid)

// TODO

// --------- GET /1/latest-import
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-latest-import

response_type! {
    /// Response type for [`Client::get_latest_import`](super::Client::get_latest_import).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct GetLatestImportResponse {
        pub latest_import: i64,
        pub musicbrainz_id: String,
    }
}

// --------- POST /1/latest-import
// https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#post--1-latest-import

response_type! {
    /// Response type for [`Client::update_latest_import`](super::Client::update_latest_import).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct UpdateLatestImportResponse {
        pub status: String,
    }
}
