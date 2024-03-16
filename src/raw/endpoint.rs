use std::fmt;

pub enum Endpoint<'a> {
    SubmitListens,
    ValidateToken,
    DeleteListen,
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
    StatsReleaseGroupListeners(&'a str),
    StatusGetDumpInfo,
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::SubmitListens => "submit-listens",
            Self::ValidateToken => "validate-token",
            Self::DeleteListen => "delete-listen",
            Self::UserListenCount(user) => return write!(f, "user/{user}/listen-count"),
            Self::UserPlayingNow(user) => return write!(f, "user/{user}/playing-now"),
            Self::UserListens(user) => return write!(f, "user/{user}/listens"),
            Self::LatestImport => "latest-import",
            Self::StatsSitewideArtists => "stats/sitewide/artists",
            Self::StatsUserListeningActivity(user) => {
                return write!(f, "stats/user/{user}/listening-activity")
            }
            Self::StatsUserDailyActivity(user) => {
                return write!(f, "stats/user/{user}/daily-activity")
            }
            Self::StatsUserRecordings(user) => return write!(f, "stats/user/{user}/recordings"),
            Self::StatsUserArtistMap(user) => return write!(f, "stats/user/{user}/artist-map"),
            Self::StatsUserReleases(user) => return write!(f, "stats/user/{user}/releases"),
            Self::StatsUserArtists(user) => return write!(f, "stats/user/{user}/artists"),
            Self::StatsReleaseGroupListeners(release_group_mbid) => return write!(f, "stats/release-group/{release_group_mbid}/listeners"),
            Self::StatusGetDumpInfo => "status/get-dump-info",
        };
        write!(f, "{s}")
    }
}
