use serde::Deserialize;
use serde::Serialize;

use crate::raw::jspf;
use crate::raw::response::response_type;

// --------- GET /1/user/(playlist_user_name)/playlists
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-user-(playlist_user_name)-playlists

response_type! {
    /// Response type for [`Client::user_playlists`](super::Client::user_playlists).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserPlaylistsResponse {
        pub count: u64,
        pub offset: u64,
        pub playlist_count: u64,
        pub playlists: Vec<jspf::Playlist>,
    }
}

// --------- GET /1/user/(playlist_user_name)/playlists/createdfor
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-user-(playlist_user_name)-playlists-createdfor

response_type! {
    /// Response type for [`Client::user_playlists_created_for`](super::Client::user_playlists_created_for).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserPlaylistsCreatedForResponse {
        pub count: u64,
        pub offset: u64,
        pub playlist_count: u64,
        pub playlists: Vec<jspf::Playlist>,
    }
}

// --------- GET /1/user/(playlist_user_name)/playlists/collaborator
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-user-(playlist_user_name)-playlists-collaborator

response_type! {
    /// Response type for [`Client::user_playlists_collaborator`](super::Client::user_playlists_collaborator).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserPlaylistsCollaboratorResponse {
        pub count: u64,
        pub offset: u64,
        pub playlist_count: u64,
        pub playlists: Vec<jspf::Playlist>,
    }
}

// --------- POST /1/playlist/create
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-create

response_type! {
    /// Response type for [`Client::playlist_create`](super::Client::playlist_create).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaylistCreateResponse {
        pub playlist_mbid: String,
        pub status: String,
    }
}

// --------- GET /1/playlist/search
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-playlist-search

// TODO

// --------- GET /1/playlist/(playlist_mbid)
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-playlist-(playlist_mbid)

response_type! {
    /// Response type for [`Client::playlist`](super::Client::get_playlist).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetPlaylistResponse {
        pub playlist: jspf::PlaylistInfo,
    }
}

// --------- GET /1/playlist/(playlist_mbid)/xspf
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-playlist-(playlist_mbid)-xspf

// TODO

// --------- POST /1/playlist/(playlist_mbid)/item/add
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-item-add
// Or
// --------- POST /1/playlist/(playlist_mbid)/item/add/(int: offset)
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-item-add-(int-offset)

// TODO

// --------- POST /1/playlist/(playlist_mbid)/item/move
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-item-move

// TODO

// --------- POST /1/playlist/(playlist_mbid)/item/delete
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-item-delete

// TODO

// --------- POST /1/playlist/(playlist_mbid)/delete
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-delete

response_type! {
    /// Response type for [`Client::playlist_delete`](super::Client::playlist_delete).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaylistDeleteResponse {
        pub status: String,
    }
}

// --------- POST /1/playlist/(playlist_mbid)/copy
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-copy

response_type! {
    /// Response type for [`Client::playlist_copy`](super::Client::playlist_copy).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct PlaylistCopyResponse {
        pub playlist_mbid: String,
        pub status: String,
    }
}

// --------- POST /1/playlist/(playlist_mbid)/export/(service)
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-(playlist_mbid)-export-(service)

// TODO

// --------- GET /1/playlist/import/(service)
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-playlist-import-(service)

// TODO

// --------- GET /1/playlist/spotify/(playlist_id)/tracks
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-playlist-spotify-(playlist_id)-tracks

// TODO

// ---------  GET /1/playlist/apple_music/(playlist_id)/tracks
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#get--1-playlist-apple_music-(playlist_id)-tracks

// TODO

// --------- POST /1/playlist/export-jspf/(service)
// https://listenbrainz.readthedocs.io/en/latest/users/api/playlist.html#post--1-playlist-export-jspf-(service)

// TODO
