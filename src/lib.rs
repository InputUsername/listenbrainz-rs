//! API bindings for ListenBrainz.

use std::fmt;
use std::io;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use ureq::Agent;

pub mod models;

use models::request::*;
use models::response::*;

const API_ROOT_URL: &str = "https://api.listenbrainz.org/1/";

enum Endpoint<'a> {
    SubmitListens,
    ValidateToken,
    DeleteListen,
    UsersRecentListens(&'a [&'a str]),
    UserListenCount(&'a str),
    UserPlayingNow(&'a str),
    UserListens(&'a str),
    LatestImport,
    // StatsSitewideArtists,
    // StatsUserListeningActivity(&'a str),
    // StatsUserDailyActivity(&'a str),
    // StatsUserRecordings(&'a str),
    // StatsUserArtistMap(&'a str),
    // StatsUserReleases(&'a str),
    // StatsUserArtists(&'a str),
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
            // Self::StatsSitewideArtists => "stats/sitewide/artists",
            // Self::StatsUserListeningActivity(user) => return write!(f, "stats/user/{}/listening-activity", user),
            // Self::StatsUserDailyActivity(user) => return write!(f, "stats/user/{}/daily-activity", user),
            // Self::StatsUserRecordings(user) => return write!(f, "stats/user/{}/recordings", user),
            // Self::StatsUserArtistMap(user) => return write!(f, "stats/user/{}/artist-map", user),
            // Self::StatsUserReleases(user) => return write!(f, "stats/user/{}/releases", user),
            // Self::StatsUserArtists(user) => return write!(f, "stats/user/{}/artists", user),
            Self::StatusGetDumpInfo => "status/get-dump-info",
        };
        write!(f, "{}", s)
    }
}

/// Represents errors that can occor while interacting with the API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The API returned a non-200 status code.
    #[error("API error ({code}): {error}")]
    Api { code: u16, error: String },

    /// The input data could not be converted into JSON.
    #[error("could not convert request input into JSON")]
    RequestJson(#[source] serde_json::Error),

    /// The HTTP response could not be converted into JSON.
    #[error("could not convert HTTP response into JSON")]
    ResponseJson(#[source] io::Error),

    /// There was some other HTTP error while interacting with the API.
    #[error("HTTP error")]
    Http(#[source] ureq::Error),
}

#[derive(Debug, Deserialize)]
struct ApiError {
    code: u16,
    error: String,
}

impl From<ApiError> for Error {
    fn from(api_error: ApiError) -> Self {
        Error::Api {
            code: api_error.code,
            error: api_error.error,
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(_code, response) => match response.into_json::<ApiError>() {
                Ok(api_error) => api_error.into(),
                Err(err) => Error::ResponseJson(err),
            },
            ureq::Error::Transport(_) => Error::Http(error),
        }
    }
}

/// Low-level client that more-or-less directly wraps the ListenBrainz HTTP API.
///
/// Client exposes functions that map one-to-one to the API methods described
/// in the [ListenBrainz API docs](https://listenbrainz.readthedocs.io/en/production/dev/api/).
pub struct Client {
    agent: Agent,
}

impl Client {
    /// Construct a new client.
    pub fn new() -> Self {
        Self {
            agent: ureq::agent(),
        }
    }

    fn get<R: DeserializeOwned>(&mut self, endpoint: Endpoint) -> Result<R, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, endpoint);

        self.agent
            .get(&endpoint)
            .call()?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    fn post<D, R>(&mut self, endpoint: Endpoint, token: &str, data: D) -> Result<R, Error>
    where
        D: Serialize,
        R: DeserializeOwned,
    {
        let data = serde_json::to_value(data).map_err(Error::RequestJson)?;

        let endpoint = format!("{}{}", API_ROOT_URL, endpoint);

        self.agent
            .post(&endpoint)
            .set("Authorization", &format!("Token {}", token))
            .send_json(data)?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Endpoint: `submit-listens`
    pub fn submit_listens(
        &mut self,
        token: &str,
        data: SubmitListens,
    ) -> Result<SubmitListensResponse, Error> {
        self.post(Endpoint::SubmitListens, token, data)
    }

    /// Endpoint: `validate-token`
    pub fn validate_token(&mut self, token: &str) -> Result<ValidateTokenResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::ValidateToken);

        self.agent
            .get(&endpoint)
            .query("token", token)
            .call()?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Endpoint: `delete-listen`
    pub fn delete_listen(
        &mut self,
        token: &str,
        data: DeleteListen,
    ) -> Result<DeleteListenResponse, Error> {
        self.post(Endpoint::DeleteListen, token, data)
    }

    /// Endpoint: `users/{user_list}/recent-listens
    pub fn users_recent_listens(
        &mut self,
        user_list: &[&str],
    ) -> Result<UsersRecentListensResponse, Error> {
        self.get(Endpoint::UsersRecentListens(user_list))
    }

    /// Endpoint: `user/{user_name}/listen-count`
    pub fn user_listen_count(&mut self, user_name: &str) -> Result<UserListenCountResponse, Error> {
        self.get(Endpoint::UserListenCount(user_name))
    }

    // UserPlayingNow(&'a str),
    /// Endpoint: `user/{user_name}/playing-now`
    pub fn user_playing_now(&mut self, user_name: &str) -> Result<UserPlayingNowResponse, Error> {
        self.get(Endpoint::UserPlayingNow(user_name))
    }

    pub fn user_listens(
        &mut self,
        user_name: &str,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        count: Option<u32>,
        time_range: Option<u64>
    ) -> Result<UserListensResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::UserListens(user_name));

        let mut request = self.agent.get(&endpoint);

        if let Some(min_ts) = min_ts {
            request = request.query("min_ts", &min_ts.to_string());
        }
        if let Some(max_ts) = max_ts {
            request = request.query("max_ts", &max_ts.to_string());
        }
        if let Some(count) = count {
            request = request.query("count", &count.to_string());
        }
        if let Some(time_range) = time_range {
            request = request.query("time_range", &time_range.to_string());
        }

        request.call()?.into_json().map_err(Error::ResponseJson)
    }

    /// Endpoint: `latest-import` (GET)
    pub fn get_latest_import(&mut self, user_name: &str) -> Result<GetLatestImportResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::LatestImport);

        self.agent
            .get(&endpoint)
            .query("user_name", user_name)
            .call()?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Endpoint: `latest-import` (POST)
    pub fn update_latest_import(
        &mut self,
        token: &str,
        data: UpdateLatestImport,
    ) -> Result<UpdateLatestImportResponse, Error> {
        self.post(Endpoint::LatestImport, token, data)
    }

    // StatsSitewideArtists,
    // StatsUserListeningActivity(&'a str),
    // StatsUserDailyActivity(&'a str),
    // StatsUserRecordings(&'a str),
    // StatsUserArtistMap(&'a str),
    // StatsUserReleases(&'a str),
    // StatsUserArtists(&'a str),

    /// Endpoint: `status/get-dump-info`
    pub fn status_get_dump_info(
        &mut self,
        id: Option<i64>,
    ) -> Result<StatusGetDumpInfoResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::StatusGetDumpInfo);

        let mut request = self.agent.get(&endpoint);

        if let Some(id) = id {
            request = request.query("id", &id.to_string());
        }

        request.call()?.into_json().map_err(Error::ResponseJson)
    }
}
