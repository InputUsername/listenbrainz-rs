use std::fmt;

pub enum Endpoint<'a> {
    SubmitListens,
    ValidateToken,
    DeleteListen,
    UsersRecentListens(&'a [&'a str]),
    UserSimilarUsers(&'a str),
    UserListenCount(&'a str),
    UserPlayingNow(&'a str),
    UserSimilarTo(&'a str, &'a str),
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
        match self {
            Self::SubmitListens => write!(f, "submit-listens"),
            Self::ValidateToken => write!(f, "validate-token"),
            Self::DeleteListen => write!(f, "delete-listen"),
            Self::UsersRecentListens(users) => {
                write!(f, "users/")?;
                for user in users.iter() {
                    let mut parts = user.split(',');
                    // Unwrap is fine since str::split always returns at least one item
                    write!(f, "{}", parts.next().unwrap())?;
                    for part in parts {
                        write!(f, "%2C{}", part)?;
                    }
                }
                write!(f, "/recent-listens")
            }
            Self::UserSimilarUsers(user) => write!(f, "user/{}/similar-users", user),
            Self::UserListenCount(user) => write!(f, "user/{}/listen-count", user),
            Self::UserPlayingNow(user) => write!(f, "user/{}/playing-now", user),
            Self::UserSimilarTo(user, other_user) => {
                write!(f, "user/{}/similar-to/{}", user, other_user)
            }
            Self::UserListens(user) => write!(f, "user/{}/listens", user),
            Self::LatestImport => write!(f, "latest-import"),
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
        }
    }
}
