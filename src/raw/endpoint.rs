use std::fmt;

pub enum Endpoint<'a> {
    SubmitListens,
    ValidateToken,
    DeleteListen,
    UserPlaylistsCollaborator(&'a str),
    UserPlaylistsCreatedFor(&'a str),
    UserSimilarUsers(&'a str),
    UserListenCount(&'a str),
    UserPlayingNow(&'a str),
    UserSimilarTo(&'a str, &'a str),
    UserPlaylists(&'a str),
    UserListens(&'a str),
    LatestImport,
    Playlist(&'a str),
    PlaylistCreate,
    PlaylistDelete(&'a str),
    PlaylistCopy(&'a str),
    StatsSitewideArtists,
    StatsUserListeningActivity(&'a str),
    StatsUserDailyActivity(&'a str),
    StatsUserRecordings(&'a str),
    StatsUserArtistMap(&'a str),
    StatsUserReleases(&'a str),
    StatsUserArtists(&'a str),
    StatusGetDumpInfo,
    UserFollowers(&'a str),
    UserFollowing(&'a str),
    UserUnfollow(&'a str),
    UserFollow(&'a str),
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SubmitListens => write!(f, "submit-listens"),
            Self::ValidateToken => write!(f, "validate-token"),
            Self::DeleteListen => write!(f, "delete-listen"),
            Self::UserPlaylistsCollaborator(user) => {
                write!(f, "user/{}/playlists/collaborator", user)
            }
            Self::UserPlaylistsCreatedFor(user) => write!(f, "user/{}/playlists/createdfor", user),
            Self::UserSimilarUsers(user) => write!(f, "user/{}/similar-users", user),
            Self::UserListenCount(user) => write!(f, "user/{}/listen-count", user),
            Self::UserPlayingNow(user) => write!(f, "user/{}/playing-now", user),
            Self::UserSimilarTo(user, other_user) => {
                write!(f, "user/{}/similar-to/{}", user, other_user)
            }
            Self::UserPlaylists(user) => write!(f, "user/{}/playlists", user),
            Self::UserListens(user) => write!(f, "user/{}/listens", user),
            Self::LatestImport => write!(f, "latest-import"),
            Self::Playlist(playlist) => write!(f, "playlist/{}", playlist),
            Self::PlaylistCreate => write!(f, "playlist/create"),
            Self::PlaylistDelete(playlist) => write!(f, "playlist/{}/delete", playlist),
            Self::PlaylistCopy(playlist) => write!(f, "playlist/{}/copy", playlist),
            Self::StatsSitewideArtists => write!(f, "stats/sitewide/artists"),
            Self::StatsUserListeningActivity(user) => {
                write!(f, "stats/user/{}/listening-activity", user)
            }
            Self::StatsUserDailyActivity(user) => {
                write!(f, "stats/user/{}/daily-activity", user)
            }
            Self::StatsUserRecordings(user) => write!(f, "stats/user/{}/recordings", user),
            Self::StatsUserArtistMap(user) => write!(f, "stats/user/{}/artist-map", user),
            Self::StatsUserReleases(user) => write!(f, "stats/user/{}/releases", user),
            Self::StatsUserArtists(user) => write!(f, "stats/user/{}/artists", user),
            Self::StatusGetDumpInfo => write!(f, "status/get-dump-info"),
            Self::UserFollowers(user) => write!(f, "user/{}/followers", user),
            Self::UserFollowing(user) => write!(f, "user/{}/following", user),
            Self::UserUnfollow(user) => write!(f, "user/{}/unfollow", user),
            Self::UserFollow(user) => write!(f, "user/{}/follow", user),
        }
    }
}
