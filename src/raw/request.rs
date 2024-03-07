//! Low-level request data models.

#![allow(missing_docs)]

use std::borrow::Borrow;

use serde::Serialize;

// --------- submit-listens

/// Request type for [`Client::submit_listens`](super::Client::submit_listens).
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct SubmitListens<'a, Track: StrType, Artist: StrType = Track, Release: StrType = Track> {
    pub listen_type: ListenType,
    pub payload: &'a [Payload<Track, Artist, Release>],
}

/// Type of the [`SubmitListens::listen_type`] field.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ListenType {
    Single,
    PlayingNow,
    Import,
}

/// Type of the [`SubmitListens::payload`] field.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Payload<Track: StrType, Artist: StrType = Track, Release: StrType = Track> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listened_at: Option<i64>,
    pub track_metadata: TrackMetadata<Track, Artist, Release>,
}

/// Type of the [`Payload::track_metadata`] field.
///
/// If [`release_name`](Self::release_name) will always be [None] and the type for `Release` cannot be inferred, set it to the [Empty] type.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TrackMetadata<Track: StrType, Artist: StrType = Track, Release: StrType = Track> {
    pub track_name: Track,
    pub artist_name: Artist,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<Release>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Map<String, serde_json::Value>>,
}

// --------- delete-listen

/// Request type for [`Client::delete_listen`](super::Client::delete_listen).
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct DeleteListen<T: StrType> {
    pub listened_at: i64,
    pub recording_msid: T,
}

// --------- latest-import (POST)

/// Request type for [`Client::update_latest_import`](super::Client::update_latest_import).
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct UpdateLatestImport {
    pub ts: i64,
}

/// Type for use in generic contexts that want a string. Technically, only [Serialize] is required by the api,
/// but the [Borrow] constraint makes working with values more convenient in non-write contexts.
pub trait StrType: Borrow<str> + Serialize {}
impl<T: Borrow<str> + Serialize> StrType for T {}

/// Dummy type to use as explicit `Release` type if [`TrackMetadata::release_name`] is [None] and its type cannot be inferred.
/// ```ignore
/// TrackMetadata {
///     ...,
///     release_name: None::<Empty>,
/// }
/// ```
/// or
/// ```ignore
/// TrackMetadata<_, _, Empty> {
///     ...,
///     release_name: None,
/// }
/// ```
#[allow(missing_debug_implementations)]
#[derive(Serialize)]
pub enum Empty {}
impl Borrow<str> for Empty {
    fn borrow(&self) -> &str {
        unreachable!("Should never be used as a value")
    }
}
