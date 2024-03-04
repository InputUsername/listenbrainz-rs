//! Types dealing with playlists in the [JSPF format], specifically, [MusicBrainz's format].
//!
//! [JSPF format]: https://xspf.org/jspf/
//! [MusicBrainz's format]: https://musicbrainz.org/doc/jspf

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// Top-level playlist type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    pub playlist: PlaylistInfo,
}

/// Type of the [`Playlist::playlist`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct PlaylistInfo {
    pub extension: PlaylistExtension,
    pub creator: String,
    pub date: String,
    pub title: String,
    pub track: Vec<Track>,
    pub identifier: String,
}

/// Type of the [`PlaylistInfo::extension`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct PlaylistExtension {
    #[serde(rename = "https://musicbrainz.org/doc/jspf#playlist")]
    pub musicbrainz: MusicBrainzPlaylistExtension,
}

/// Type of the [`PlaylistExtension::musicbrainz`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct MusicBrainzPlaylistExtension {
    pub created_for: String,
    pub creator: String,
    #[serde(default)]
    pub collaborators: Vec<String>,
    pub copied_from: Option<String>,
    #[serde(default)]
    pub copied_from_deleted: bool,
    pub public: bool,
    pub last_modified_at: String,
    pub additional_metadata: AdditionalMetadata,
}

/// Type of the [`MusicBrainzPlaylistExtension::additional_metadata`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct AdditionalMetadata {
    pub algorithm_metadata: Option<AlgorithmMetadata>,
}

/// Type of the [`AdditionalMetadata::algorithm_metadata`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct AlgorithmMetadata {
    pub source_patch: Option<String>,
}

/// Type of the [`PlaylistInfo::track`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub title: String,
    pub identifier: String,
    pub creator: String,
    pub extension: TrackExtension,
    pub album: String,
}

/// Type of the [`Track::extension`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct TrackExtension {
    #[serde(rename = "https://musicbrainz.org/doc/jspf#track")]
    pub musicbrainz: MusicBrainzTrackExtension,
}

/// Type of the [`TrackExtension::musicbrainz`] field.
#[derive(Debug, Deserialize, Serialize)]
pub struct MusicBrainzTrackExtension {
    pub added_by: String,
    #[serde(default)]
    pub artist_mbids: Vec<String>,
    pub added_at: String,
    pub release_identifier: Option<String>,
}
