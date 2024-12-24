//! Low-level request data models.

#![allow(missing_docs)]

use std::borrow::Borrow;

use serde::{Deserialize, Serialize};

use super::jspf::AdditionalMetadata;

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

// --------- playlist/create

/// Request type for [`Client::playlist_create`](super::Client::playlist_create).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistCreate {
    pub playlist: PlaylistCreatePlaylist,
}

/// Inner type for [`PlaylistCreate`]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistCreatePlaylist {
    pub title: String,
    pub annotation: Option<String>,
    pub track: Vec<PlaylistCreatePlaylistTrack>,
    pub extension: PlaylistCreatePlaylistExtension,
}

/// A track of the playlist for [`PlaylistCreatePlaylist`]
///
/// The identifier part of the track must be a MusicBrainz URI:
/// ```
/// use listenbrainz::raw::request::PlaylistCreatePlaylistTrack;
/// PlaylistCreatePlaylistTrack {
///     identifier: "8f3471b5-7e6a-48da-86a9-c1c07a0f47ae".to_string() // ❌ Invalid!
/// };
///
/// PlaylistCreatePlaylistTrack {
///     identifier: "https://musicbrainz.org/recording/8f3471b5-7e6a-48da-86a9-c1c07a0f47ae".to_string() // ✔️ Valid!
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistCreatePlaylistTrack {
    pub identifier: String,
}

/// The extension of [`PlaylistCreatePlaylist`]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistCreatePlaylistExtension {
    #[serde(rename = "https://musicbrainz.org/doc/jspf#playlist")]
    pub musicbrainz: PlaylistCreatePlaylistExtensionInner,
}

/// Inner part of [`PlaylistCreatePlaylistExtension`]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PlaylistCreatePlaylistExtensionInner {
    pub created_for: Option<String>,
    pub creator: Option<String>,
    pub collaborators: Vec<String>,
    pub copied_from: Option<String>,
    pub copied_from_deleted: Option<bool>,
    pub public: bool,
    pub last_modified_at: Option<String>,
    pub additional_metadata: Option<AdditionalMetadata>,
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
