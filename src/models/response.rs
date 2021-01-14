//! Low-level response data models.

use std::collections::HashMap;

use serde::Deserialize;

// --------- submit-listens

/// Response type for [`Client::submit_listens`](crate::Client::submit_listens).
#[derive(Debug, Deserialize)]
pub struct SubmitListensResponse {
    pub status: String,
}

// --------- validate-token

/// Response type for [`Client::validate_token`](crate::Client::validate_token).
#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponse {
    pub code: u16,
    pub message: String,

    pub valid: bool,
    pub user_name: Option<String>,
}

// --------- delete-listen

/// Response type for [`Client::delete_listen`](crate::Client::delete_listen).
#[derive(Debug, Deserialize)]
pub struct DeleteListenResponse {
    pub status: String,
}

// --------- users/{user_list}/recent-listens

/// Response type for [`Client::users_recent_listens`](crate::Client::users_recent_listens).
#[derive(Debug, Deserialize)]
pub struct UsersRecentListensResponse {
    pub payload: UsersRecentListensPayload,
}

/// Type of the [`UsersRecentListensResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct UsersRecentListensPayload {
    pub count: u64,
    pub listens: Vec<UsersRecentListensListen>,
    pub user_list: String,
}

/// Type of the [`UsersRecentListensPayload::listens`] field.
#[derive(Debug, Deserialize)]
pub struct UsersRecentListensListen {
    pub user_name: String,
    pub inserted_at: String,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UsersRecentListensTrackMetadata,
}

/// Type of the [`UsersRecentListensListen::track_metadata`] field.
#[derive(Debug, Deserialize)]
pub struct UsersRecentListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// --------- user/{user_name}/listen-count

/// Response type for [`Client::user_listen_count`](crate::Client::user_listen_count).
#[derive(Debug, Deserialize)]
pub struct UserListenCountResponse {
    pub payload: UserListenCountPayload,
}

#[derive(Debug, Deserialize)]
pub struct UserListenCountPayload {
    pub count: u64,
}

// -------- user/{user_name}/playing-now

/// Response type for [`Client::user_playing_now`](crate::Client::user_playing_now).
#[derive(Debug, Deserialize)]
pub struct UserPlayingNowResponse {
    pub payload: UserPlayingNowPayload,
}

/// Type of the [`UserPlayingNowResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct UserPlayingNowPayload {
    pub count: u8,
    pub user_id: String,
    pub listens: Vec<UserPlayingNowListen>,
}

/// Type of the [`UserPlayingNowPayload::listens`] field.
#[derive(Debug, Deserialize)]
pub struct UserPlayingNowListen {
    pub user_name: String,
    pub inserted_at: String,
    pub recording_msid: String,
    pub track_metadata: UserPlayingNowTrackMetadata,
}

/// Type of the [`UserPlayingNowListen::track_metadata`] field.
#[derive(Debug, Deserialize)]
pub struct UserPlayingNowTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// -------- user/{user_name}/listens

/// Response type for [`Client::user_listens`](crate::Client::user_listens).
#[derive(Debug, Deserialize)]
pub struct UserListensResponse {
    pub payload: UserListensPayload,
}

/// Type of the [`UserListensResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct UserListensPayload {
    pub count: u64,
    pub latest_listen_ts: i64,
    pub user_id: String,
    pub listens: Vec<UserListensListen>,
}

/// Type of the [`UserListensPayload::listens`] field.
#[derive(Debug, Deserialize)]
pub struct UserListensListen {
    pub user_name: String,
    pub inserted_at: String,
    pub listened_at: i64,
    pub recording_msid: String,
    pub track_metadata: UserListensTrackMetadata,
}

/// Type of the [`UserListensListen::track_metadata`] field.
#[derive(Debug, Deserialize)]
pub struct UserListensTrackMetadata {
    pub artist_name: String,
    pub track_name: String,
    pub release_name: Option<String>,
    pub additional_info: HashMap<String, serde_json::Value>,
}

// --------- latest-import (GET)

/// Response type for [`Client::get_latest_import`](crate::Client::get_latest_import).
#[derive(Debug, Deserialize)]
pub struct GetLatestImportResponse {
    pub latest_import: i64,
    pub musicbrainz_id: String,
}

// --------- latest-import (POST)

/// Response type for [`Client::update_latest_import`](crate::Client::update_latest_import).
#[derive(Debug, Deserialize)]
pub struct UpdateLatestImportResponse {
    pub status: String,
}

// --------- stats/sitewide/artists

/// Response type for [`Client::stats_sitewide_artists`](crate::Client::stats_sitewide_artists).
#[derive(Debug, Deserialize)]
pub struct StatsSitewideArtistsResponse {
    pub payload: StatsSitewideArtistsPayload,
}

/// Type of the [`StatsSitewideArtistsResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct StatsSitewideArtistsPayload {
    pub time_ranges: Vec<StatsSitewideArtistsTimeRange>,
    pub offset: u64,
    pub count: u64,
    pub range: String,
    pub last_updated: i64,
    pub from_ts: i64,
    pub to_ts: i64,
}

/// Type of the [`StatsSitewideArtistsPayload::time_ranges`] field.
#[derive(Debug, Deserialize)]
pub struct StatsSitewideArtistsTimeRange {
    pub time_range: String,
    pub artists: Vec<StatsSitewideArtistsArtist>,
    pub from_ts: i64,
    pub to_ts: i64,
}

/// Type of the [`StatsSitewideArtistsTimeRange::artists`] field.
#[derive(Debug, Deserialize)]
pub struct StatsSitewideArtistsArtist {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: String,
    pub artist_name: String,
    pub listen_count: u64,
}

// --------- stats/user/{user_name}/listening-activity

/// Response type for [`Client::stats_user_listening_activity`](crate::Client::stats_user_listening_activity).
#[derive(Debug, Deserialize)]
pub struct StatsUserListeningActivityResponse {
    pub payload: StatsUserListeningActivityPayload,
}

/// Type of the [`StatsUserListeningActivityResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserListeningActivityPayload {
    pub user_id: String,
    pub listening_activity: Vec<StatsUserListeningActivityListeningActivity>,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
}

/// Type of the [`StatsUserListeningActivityPayload::listening_activity`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserListeningActivityListeningActivity {
    pub listen_count: u64,
    pub from_ts: i64,
    pub to_ts: i64,
    pub time_range: String,
}

// --------- stats/user/{user_name}/daily-activity

/// Response type for [`Client::stats_user_daily_activity`](crate::Client::stats_user_daily_activity).
#[derive(Debug, Deserialize)]
pub struct StatsUserDailyActivityResponse {
    pub payload: StatsUserDailyActivityPayload,
}

/// Type of the [`StatsUserDailyActivityResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserDailyActivityPayload {
    pub user_id: String,
    pub daily_activity: StatsUserDailyActivityDailyActivity,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub stats_range: String,
}

/// Type of the [`StatsUserDailyActivityPayload::daily_activity`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserDailyActivityDailyActivity {
    pub days: HashMap<String, Vec<StatsUserDailyActivityHour>>,
}

/// Type of the [`StatsUserDailyActivityDailyActivity::days`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserDailyActivityHour {
    pub hour: u8,
    pub listen_count: u64,
}

// --------- stats/user/{user_name}/recordings

/// Response type of [`Client::stats_user_recordings`](crate::Client::stats_user_recordings).
#[derive(Debug, Deserialize)]
pub struct StatsUserRecordingsResponse {
    pub payload: StatsUserRecordingsPayload,
}

/// Type of the [`StatsUserRecordingsResponse::payload`] field.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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

/// Response type of [`Client::stats_user_artist_map`](crate::Client::stats_user_artist_map).
#[derive(Debug, Deserialize)]
pub struct StatsUserArtistMapResponse {
    pub payload: StatsUserArtistMapPayload,
}

/// Type of the [`StatsUserArtistMapResponse::payload`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserArtistMapPayload {
    pub artist_map: Vec<StatsUserArtistMapCountry>,
    pub user_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub range: String,
}

/// Type of the [`StatsUserArtistMapPayload::artist_map`] field.
#[derive(Debug, Deserialize)]
pub struct StatsUserArtistMapCountry {
    pub country: String,
    pub artist_count: u64,
}

// --------- stats/user/{user_name}/releases

/// Response type for [`Client::stats_user_releases`](crate::Client::stats_user_releases).
#[derive(Debug, Deserialize)]
pub struct StatsUserReleasesResponse {
    pub payload: StatsUserReleasesPayload,
}

/// Type of the [`StatsUserReleasesResponse::payload`] field.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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

/// Response type of [`Client::stats_user_artists`](crate::Client::stats_user_artists).
#[derive(Debug, Deserialize)]
pub struct StatsUserArtistsResponse {
    pub payload: StatsUserArtistsPayload,
}

/// Type of the [`StatsUserArtistsResponse::payload`] field.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct StatsUserArtistsArtist {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: Option<String>,
    pub artist_name: String,
    pub listen_count: u64,
}

// --------- status/get-dump-info

/// Response type for [`Client::status_get_dump_info`](crate::Client::status_get_dump_info).
#[derive(Debug, Deserialize)]
pub struct StatusGetDumpInfoResponse {
    pub code: u16,
    pub message: String,

    pub id: i64,
    pub timestamp: String,
}
