use std::fmt;

pub enum Endpoint<'a> {
    SubmitListens,
    ValidateToken,
    DeleteListen,
    UsersRecentListens(&'a [&'a str]),
    UserListenCount(&'a str),
    UserPlayingNow(&'a str),
    UserListens(&'a str),
    LatestImport,
    StatsSitewideArtists,
    StatsUserListeningActivity(&'a str),
    StatsUserDailyActivity(&'a str),
    StatsUserRecordings(&'a str),
    StatsUserArtistMap(&'a str),
    StatsUserReleases(&'a str),
    StatsUserArtists(&'a str),
    StatusGetDumpInfo,
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::SubmitListens => "submit-listens",
            Self::ValidateToken => "validate-token",
            Self::DeleteListen => "delete-listen",
            Self::UsersRecentListens(users) => {
                // TODO: url-encode usernames with commas
                return write!(f, "users/{}/recent-listens", users.join(","));
            }
            Self::UserListenCount(user) => return write!(f, "user/{}/listen-count", user),
            Self::UserPlayingNow(user) => return write!(f, "user/{}/playing-now", user),
            Self::UserListens(user) => return write!(f, "user/{}/listens", user),
            Self::LatestImport => "latest-import",
            Self::StatsSitewideArtists => "stats/sitewide/artists",
            Self::StatsUserListeningActivity(user) => return write!(f, "stats/user/{}/listening-activity", user),
            Self::StatsUserDailyActivity(user) => return write!(f, "stats/user/{}/daily-activity", user),
            Self::StatsUserRecordings(user) => return write!(f, "stats/user/{}/recordings", user),
            Self::StatsUserArtistMap(user) => return write!(f, "stats/user/{}/artist-map", user),
            Self::StatsUserReleases(user) => return write!(f, "stats/user/{}/releases", user),
            Self::StatsUserArtists(user) => return write!(f, "stats/user/{}/artists", user),
            Self::StatusGetDumpInfo => "status/get-dump-info",
        };
        write!(f, "{}", s)
    }
}
