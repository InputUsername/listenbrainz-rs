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

// --- Art Requests ---
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ArtGrid {
    /// The background for the cover art
    pub background: ArtGridBackground,

    /// The size of the cover art image
    pub image_size: u64,

    /// The dimension to use for this grid. A grid of dimension 3 has 3 images across and 3 images down, for a total of 9 images.
    pub dimension: u64,

    /// If cover art is missing for a given release_mbid, skip it and move on to the next one, if true is passed.
    /// If false, the show-caa option will decide what happens.
    #[serde(rename = "skip-missing")]
    pub skip_missing: bool,

    /// If cover art is missing and skip-missing is false,
    /// then show-caa will determine if a blank square is shown or if the Cover Art Archive missing image is shown.
    #[serde(rename = "show-caa")]
    pub show_caa: bool,

    /// The tiles parameter is a list of strings that determines the location where cover art images should be placed.
    /// Each string is a comma separated list of image cells.
    /// A grid of dimension 3 has 9 cells, from 0 in the upper left hand corner, 2 in the upper right hand corner,
    /// 6 in the lower left corner and 8 in the lower right corner.
    ///
    /// ```plaintext
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    ///
    /// Specifying only a single cell will have the image cover that cell exactly.
    /// If more than one cell is specified, the image will cover the area defined by the bounding box of all the given cells.
    /// These tiles only define bounding box areas – no clipping of images that may fall outside of these tiles will be performed.
    ///
    /// ## Exemple
    /// For this exemple, the dimension is set to 3
    /// ```
    /// let tiles = vec![
    ///     "0, 1, 3, 4".to_string(), // The first image cover the slots 0, 1, 3, and 4
    ///     "2".to_string(),
    ///     "5".to_string(),
    ///     "6".to_string(),
    ///     "7".to_string(),
    ///     "8".to_string(),
    /// ];
    /// ```
    ///
    /// This will result in this arrangement:
    ///
    /// ```plaintext
    /// 1 1 2
    /// 1 1 3
    /// 4 5 6
    /// ```
    pub tiles: Option<Vec<String>>,

    /// An ordered list of release_mbids. The images will be loaded and processed in the order that this list is in.
    /// The cover art for the release_mbids will be placed on the tiles defined by the tiles parameter.
    /// If release_group_mbids are supplied as well, ONLY cover arts for release_group_mbids will be processed.
    pub release_mbids: Option<Vec<String>>,

    /// An ordered list of release_group_mbids. The images will be loaded and processed in the order that this list is in.
    /// The cover art for the release_group_mbids will be placed on the tiles defined by the tiles parameter.
    /// If release_mbids are supplied as well, ONLY cover arts for release_mbids will be processed.
    pub release_group_mbids: Option<Vec<String>>,

    /// Size in pixels of each cover art in the composited image. Can be either 250 or 500
    pub cover_art_size: Option<u64>,
}

/// The background for the cover art
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ArtGridBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}
