//! Low-level response data models.
//!
//! Every response type has the `rate_limit` field, which contains rate limiting
//! information. See the documentation of the [`RateLimit`] type for more
//! details.

#![allow(missing_docs)]

use std::collections::HashMap;

use attohttpc::Response;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::Error;

/// Contains rate limiting information.
///
/// ListenBrainz API rate limiting is described in the [API docs].
/// Prefer using the [`RateLimit::reset_in`] field over [`RateLimit::reset`],
/// as the former is resilient against clients with incorrect clocks.
///
/// [API docs]: https://listenbrainz.readthedocs.io/en/production/dev/api/#rate-limiting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimit {
    pub limit: u64,
    pub remaining: u64,
    pub reset_in: u64,
    pub reset: i64,
}

impl RateLimit {
    /// Extract rate limiting information from the `X-RateLimit-` headers.
    /// Only returns `Some` if all fields are present and valid.
    fn from_headers(response: &Response) -> Option<Self> {
        let headers = response.headers();

        let limit = headers
            .get("X-RateLimit-Limit")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;
        let remaining = headers
            .get("X-RateLimit-Remaining")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;
        let reset_in = headers
            .get("X-RateLimit-Reset-In")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;
        let reset = headers
            .get("X-RateLimit-Reset")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;

        Some(Self {
            limit,
            remaining,
            reset_in,
            reset,
        })
    }
}

/// Internal trait for response types.
/// Allows converting the response type from an `attohttpc::Response`,
/// by deserializing the body into the response type and then
/// adding the `rate_limit` field from headers.
pub(crate) trait ResponseType: DeserializeOwned {
    fn from_response(response: Response) -> Result<Self, Error>;
}

/// Internal macro for response types.
/// Wraps the definition of a response type, adds the `rate_limit` field,
/// and implements the `ResponseType` trait.
macro_rules! response_type {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field:ident: $field_ty:ty),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        pub struct $name {
            #[serde(skip)]
            pub rate_limit: Option<RateLimit>,
            $(pub $field: $field_ty),*
        }

        impl ResponseType for $name {
            fn from_response(response: Response) -> Result<Self, Error> {
                let response = Error::try_from_error_response(response)?;
                let rate_limit = RateLimit::from_headers(&response);
                let mut result: Self = response.json()?;
                result.rate_limit = rate_limit;
                Ok(result)
            }
        }
    }
}

// --------- submit-listens

response_type! {
    /// Response type for [`Client::submit_listens`](super::Client::submit_listens).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct SubmitListensResponse {
        pub status: String,
    }
}

// --------- validate-token

response_type! {
    /// Response type for [`Client::validate_token`](super::Client::validate_token).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct ValidateTokenResponse {
        pub code: u16,
        pub message: String,

        pub valid: bool,
        pub user_name: Option<String>,
    }
}

// --------- delete-listen

response_type! {
    /// Response type for [`Client::delete_listen`](super::Client::delete_listen).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct DeleteListenResponse {
        pub status: String,
    }
}

// --------- user/{user_name}/listen-count

response_type! {
    /// Response type for [`Client::user_listen_count`](super::Client::user_listen_count).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct UserListenCountResponse {
        pub payload: UserListenCountPayload,
    }
}

/// Type of the [`UserListenCountResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserListenCountPayload {
    pub count: u64,
}

// -------- user/{user_name}/playing-now

response_type! {
    /// Response type for [`Client::user_playing_now`](super::Client::user_playing_now).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct UserPlayingNowResponse {
        pub payload: UserPlayingNowPayload,
    }
}

/// Type of the [`UserPlayingNowResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserPlayingNowPayload {
    pub count: u8,
    pub user_id: String,
    pub listens: Vec<UserPlayingNowListen>,
    pub playing_now: bool,
}

/// Type of the [`UserPlayingNowPayload::listens`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserPlayingNowListen {
    pub track_metadata: UserPlayingNowTrackMetadata,
    pub playing_now: bool,
}

/// Type of the [`UserPlayingNowListen::track_metadata`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserPlayingNowTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// -------- user/{user_name}/listens

response_type! {
    /// Response type for [`Client::user_listens`](super::Client::user_listens).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct UserListensResponse {
        pub payload: UserListensPayload,
    }
}

/// Type of the [`UserListensResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserListensPayload {
    pub count: u64,
    pub latest_listen_ts: i64,
    pub oldest_listen_ts: i64,
    pub user_id: String,
    pub listens: Vec<UserListensListen>,
}

/// Type of the [`UserListensPayload::listens`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserListensListen {
    pub user_name: String,
    pub inserted_at: i64,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UserListensTrackMetadata,
}

/// Type of the [`UserListensListen::track_metadata`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
    pub mbid_mapping: Option<UserListensMBIDMapping>,
}

/// Type of the [`UserListensTrackMetadata::mbid_mapping`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserListensMBIDMapping {
    pub artist_mbids: Option<Vec<String>>,
    pub artists: Option<Vec<UserListensMappingArtist>>,
    pub recording_mbid: String,
    pub recording_name: Option<String>,
}

/// Type of the [`UserListensMBIDMapping::artists`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct UserListensMappingArtist {
    pub artist_mbid: String,
    pub artist_credit_name: String,
    pub join_phrase: String,
}

// --------- latest-import (GET)

response_type! {
    /// Response type for [`Client::get_latest_import`](super::Client::get_latest_import).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct GetLatestImportResponse {
        pub latest_import: i64,
        pub musicbrainz_id: String,
    }
}

// --------- latest-import (POST)

response_type! {
    /// Response type for [`Client::update_latest_import`](super::Client::update_latest_import).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct UpdateLatestImportResponse {
        pub status: String,
    }
}

// --------- stats/sitewide/artists

response_type! {
    /// Response type for [`Client::stats_sitewide_artists`](super::Client::stats_sitewide_artists).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsSitewideArtistsResponse {
        pub payload: StatsSitewideArtistsPayload,
    }
}

/// Type of the [`StatsSitewideArtistsResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsSitewideArtistsPayload {
    pub artists: Vec<StatsSitewideArtistsArtist>,
    pub offset: u64,
    pub count: u64,
    pub range: String,
    pub last_updated: i64,
    pub from_ts: i64,
    pub to_ts: i64,
}

/// Type of the [`StatsSitewideArtistsTimeRange::artists`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsSitewideArtistsArtist {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_name: String,
    pub listen_count: u64,
}

// --------- stats/user/{user_name}/listening-activity

response_type! {
    /// Response type for [`Client::stats_user_listening_activity`](super::Client::stats_user_listening_activity).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsUserListeningActivityResponse {
        pub payload: StatsUserListeningActivityPayload,
    }
}

/// Type of the [`StatsUserListeningActivityResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserListeningActivityPayload {
    pub user_id: String,
    pub listening_activity: Vec<StatsUserListeningActivityListeningActivity>,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
}

/// Type of the [`StatsUserListeningActivityPayload::listening_activity`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserListeningActivityListeningActivity {
    pub listen_count: u64,
    pub from_ts: i64,
    pub to_ts: i64,
    pub time_range: String,
}

// --------- stats/user/{user_name}/daily-activity

response_type! {
    /// Response type for [`Client::stats_user_daily_activity`](super::Client::stats_user_daily_activity).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsUserDailyActivityResponse {
        pub payload: StatsUserDailyActivityPayload,
    }
}

/// Type of the [`StatsUserDailyActivityResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserDailyActivityPayload {
    pub user_id: String,
    pub daily_activity: StatsUserDailyActivityDailyActivity,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub stats_range: String,
}

/// Type of the [`StatsUserDailyActivityPayload::daily_activity`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserDailyActivityDailyActivity {
    pub days: HashMap<String, Vec<StatsUserDailyActivityHour>>,
}

/// Type of the [`StatsUserDailyActivityDailyActivity::days`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserDailyActivityHour {
    pub hour: u8,
    pub listen_count: u64,
}

// --------- stats/user/{user_name}/recordings

response_type! {
    /// Response type of [`Client::stats_user_recordings`](super::Client::stats_user_recordings).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsUserRecordingsResponse {
        pub payload: StatsUserRecordingsPayload,
    }
}

/// Type of the [`StatsUserRecordingsResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserRecordingsPayload {
    pub recordings: Vec<StatsUserRecordingsRecording>,
    pub count: u64,
    pub total_recording_count: u64,
    pub user_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub range: String,
}

/// Type of the [`StatsUserRecordingsPayload::recordings`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserRecordingsRecording {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: Option<String>,
    pub artist_name: String,
    pub listen_count: u64,
    pub recording_mbid: Option<String>,
    pub recording_msid: Option<String>,
    pub release_mbid: Option<String>,
    pub release_msid: Option<String>,
    pub release_name: Option<String>,
    pub track_name: Option<String>,
}

// --------- stats/user/{user_name}/artist-map

response_type! {
    /// Response type of [`Client::stats_user_artist_map`](super::Client::stats_user_artist_map).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsUserArtistMapResponse {
        pub payload: StatsUserArtistMapPayload,
    }
}

/// Type of the [`StatsUserArtistMapResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistMapPayload {
    pub artist_map: Vec<StatsUserArtistMapCountry>,
    pub user_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub range: String,
}

/// Type of the [`StatsUserArtistMapPayload::artist_map`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistMapCountry {
    pub country: String,
    pub artist_count: u64,
}

// --------- stats/user/{user_name}/releases

response_type! {
    /// Response type for [`Client::stats_user_releases`](super::Client::stats_user_releases).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsUserReleasesResponse {
        pub payload: StatsUserReleasesPayload,
    }
}

/// Type of the [`StatsUserReleasesResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserReleasesPayload {
    pub releases: Vec<StatsUserReleasesRelease>,
    pub count: u64,
    pub total_release_count: u64,
    pub user_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub range: String,
}

/// Type of the [`StatsUserReleasesPayload::releases`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserReleasesRelease {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: Option<String>,
    pub artist_name: String,
    pub listen_count: u64,
    pub release_mbid: Option<String>,
    pub release_msid: Option<String>,
    pub release_name: String,
}

// --------- stats/user/{user_name}/artists

response_type! {
    /// Response type of [`Client::stats_user_artists`](super::Client::stats_user_artists).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatsUserArtistsResponse {
        pub payload: StatsUserArtistsPayload,
    }
}

/// Type of the [`StatsUserArtistsResponse::payload`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistsPayload {
    pub artists: Vec<StatsUserArtistsArtist>,
    pub count: u64,
    pub total_artist_count: u64,
    pub user_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub range: String,
}

/// Type of the [`StatsUserArtistsPayload::artists`] field.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistsArtist {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: Option<String>,
    pub artist_name: String,
    pub listen_count: u64,
}

// --------- status/get-dump-info

response_type! {
    /// Response type for [`Client::status_get_dump_info`](super::Client::status_get_dump_info).
    #[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
    pub struct StatusGetDumpInfoResponse {
        pub code: u16,
        pub message: String,

        pub id: i64,
        pub timestamp: String,
    }
}
