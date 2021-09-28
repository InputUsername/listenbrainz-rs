//! Low-level request data models.

use std::collections::HashMap;

use serde::Serialize;

// --------- submit-listens

/// Request type for [`Client::submit_listens`](super::Client::submit_listens).
#[derive(Debug, Serialize)]
pub struct SubmitListens<'a> {
    pub listen_type: ListenType,
    pub payload: &'a [Payload<'a>],
}

/// Type of the [`SubmitListens::listen_type`] field.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListenType {
    Single,
    PlayingNow,
    Import,
}

/// Type of the [`SubmitListens::payload`] field.
#[derive(Debug, Serialize)]
pub struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listened_at: Option<i64>,
    pub track_metadata: TrackMetadata<'a>,
}

/// Type of the [`Payload::track_metadata`] field.
#[derive(Debug, Serialize)]
pub struct TrackMetadata<'a> {
    pub artist_name: &'a str,
    pub track_name: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<HashMap<&'a str, serde_json::Value>>,
}

// --------- delete-listen

/// Request type for [`Client::delete_listen`](super::Client::delete_listen).
#[derive(Debug, Serialize)]
pub struct DeleteListen<'a> {
    pub listened_at: i64,
    pub recording_msid: &'a str,
}

// --------- latest-import (POST)

/// Request type for [`Client::update_latest_import`](super::Client::update_latest_import).
#[derive(Debug, Serialize)]
pub struct UpdateLatestImport {
    pub ts: i64,
}

// --------- playlist/{playlist_mbid}/item/delete

/// Request type for [`Client::playlist_item_delete`](super::Client::playlist_item_delete).
#[derive(Debug, Serialize)]
pub struct PlaylistItemDelete {
    pub index: u64,
    pub count: u64,
}

// --------- playlist/{playlist_mbid}/item/move

/// Request type for [`Client::playlist_item_move`](super::Client::playlist_item_move).
#[derive(Debug, Serialize)]
pub struct PlaylistItemMove<'a> {
    pub mbid: &'a str,
    pub from: u64,
    pub to: u64,
    pub count: u64,
}
