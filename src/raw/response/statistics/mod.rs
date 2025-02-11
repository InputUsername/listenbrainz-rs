use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::raw::response::response_type;

// --------- GET /1/stats/user/(user_name)/artists
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-user-(user_name)-artists

response_type! {
    /// Response type of [`Client::stats_user_artists`](super::Client::stats_user_artists).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsUserArtistsResponse {
        pub payload: StatsUserArtistsPayload,
    }
}

/// Type of the [`StatsUserArtistsResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistsArtist {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: Option<String>,
    pub artist_name: String,
    pub listen_count: u64,
}

// --------- GET /1/stats/user/(user_name)/releases
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-user-(user_name)-releases

response_type! {
    /// Response type for [`Client::stats_user_releases`](super::Client::stats_user_releases).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsUserReleasesResponse {
        pub payload: StatsUserReleasesPayload,
    }
}


/// Type of the [`StatsUserReleasesResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserReleasesRelease {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_msid: Option<String>,
    pub artist_name: String,
    pub listen_count: u64,
    pub release_mbid: Option<String>,
    pub release_msid: Option<String>,
    pub release_name: String,
}

// --------- GET /1/stats/user/(user_name)/recordings
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-user-(user_name)-recordings

response_type! {
    /// Response type of [`Client::stats_user_recordings`](super::Client::stats_user_recordings).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsUserRecordingsResponse {
        pub payload: StatsUserRecordingsPayload,
    }
}

/// Type of the [`StatsUserRecordingsResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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

// --------- GET /1/stats/user/(user_name)/listening-activity
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-user-(user_name)-listening-activity

response_type! {
    /// Response type for [`Client::stats_user_listening_activity`](super::Client::stats_user_listening_activity).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsUserListeningActivityResponse {
        pub payload: StatsUserListeningActivityPayload,
    }
}

/// Type of the [`StatsUserListeningActivityResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserListeningActivityPayload {
    pub user_id: String,
    pub listening_activity: Vec<StatsUserListeningActivityListeningActivity>,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
}

/// Type of the [`StatsUserListeningActivityPayload::listening_activity`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserListeningActivityListeningActivity {
    pub listen_count: u64,
    pub from_ts: i64,
    pub to_ts: i64,
    pub time_range: String,
}

// --------- GET /1/stats/user/(user_name)/daily-activity
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-user-(user_name)-daily-activity

response_type! {
    /// Response type for [`Client::stats_user_daily_activity`](super::Client::stats_user_daily_activity).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsUserDailyActivityResponse {
        pub payload: StatsUserDailyActivityPayload,
    }
}

/// Type of the [`StatsUserDailyActivityResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserDailyActivityPayload {
    pub user_id: String,
    pub daily_activity: StatsUserDailyActivityDailyActivity,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub stats_range: String,
}

/// Type of the [`StatsUserDailyActivityPayload::daily_activity`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserDailyActivityDailyActivity {
    pub days: HashMap<String, Vec<StatsUserDailyActivityHour>>,
}

/// Type of the [`StatsUserDailyActivityDailyActivity::days`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserDailyActivityHour {
    pub hour: u8,
    pub listen_count: u64,
}

// --------- GET /1/stats/user/(user_name)/artist-map
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-user-(user_name)-artist-map

response_type! {
    /// Response type of [`Client::stats_user_artist_map`](super::Client::stats_user_artist_map).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsUserArtistMapResponse {
        pub payload: StatsUserArtistMapPayload,
    }
}

/// Type of the [`StatsUserArtistMapResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistMapPayload {
    pub artist_map: Vec<StatsUserArtistMapCountry>,
    pub user_id: String,
    pub from_ts: i64,
    pub to_ts: i64,
    pub last_updated: i64,
    pub range: String,
}

/// Type of the [`StatsUserArtistMapPayload::artist_map`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsUserArtistMapCountry {
    pub country: String,
    pub artist_count: u64,
}

// --------- GET /1/stats/release-group/(release_group_mbid)/listeners
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-release-group-(release_group_mbid)-listeners

response_type! {
    /// Response type for [`Client::stats_release_group_listeners`](super::Client::stats_release_group_listeners).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct StatsReleaseGroupListenersResponse {
        pub payload: StatsReleaseGroupListenersPayload
    }
}

/// Type of the [`StatsReleaseGroupListenersResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsReleaseGroupListenersPayload {
    pub artist_mbids: Vec<String>,
    pub artist_name: String,
    pub caa_id: Option<i64>,
    pub caa_release_mbid: Option<String>,
    pub from_ts: i64,
    pub last_updated: i64,
    pub listeners: Vec<StatsReleaseGroupListenersListeners>,
    pub release_group_mbid: String,
    pub release_group_name: String,
    pub stats_range: String,
    pub to_ts: i64,
    pub total_listen_count: i64,
}

/// Type of the [`StatsReleaseGroupListenersPayload::listeners`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsReleaseGroupListenersListeners {
    pub listen_count: u64,
    pub username_name: String,
}

// --------- GET /1/stats/sitewide/artists
// https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-sitewide-artists
response_type! {
    /// Response type for [`Client::stats_sitewide_artists`](super::Client::stats_sitewide_artists).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatsSitewideArtistsResponse {
        pub payload: StatsSitewideArtistsPayload,
    }
}

/// Type of the [`StatsSitewideArtistsResponse::payload`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsSitewideArtistsPayload {
    pub artists: Vec<StatsSitewideArtistsArtist>,
    pub offset: u64,
    pub count: u64,
    pub range: String,
    pub last_updated: i64,
    pub from_ts: i64,
    pub to_ts: i64,
}

/// Type of the [`StatsSitewideArtistsPayload::artists`] field.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct StatsSitewideArtistsArtist {
    pub artist_mbids: Option<Vec<String>>,
    pub artist_name: String,
    pub listen_count: u64,
}