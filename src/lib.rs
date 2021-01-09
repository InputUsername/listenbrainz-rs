//! API bindings for ListenBrainz.

use std::fmt;
use std::io;

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use thiserror::Error;
use ureq::Agent;

pub mod models;

use models::request::*;
use models::response::*;

const API_ROOT_URL: &str = "https://api.listenbrainz.org/1/";

enum Endpoint<'a> {
    SubmitListens,
    ValidateToken,
    DeleteListen,
    //UsersRecentListens(&'a [&'a str]),
    UserListenCount(&'a str),
    // UserPlayingNow(&'a str),
    // UserListens(&'a str),
    // LatestImport,
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
            // Self::UsersRecentListens(users) => {
            //     let users = users.join(",");
            //     return write!(f, "users/{}/recent-listens", users);
            // }
            Self::UserListenCount(user) => return write!(f, "user/{}/listen-count", user),
            // Self::UserPlayingNow(user) => return write!(f, "user/{}/playing-now", user),
            // Self::UserListens(user) => return write!(f, "user/{}/listens", user),
            // Self::LatestImport => "latest-import",
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
#[derive(Error, Debug)]
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
            ureq::Error::Status(_code, response) => {
                match response.into_json::<ApiError>() {
                    Ok(api_error) => api_error.into(),
                    Err(err) => Error::ResponseJson(err),
                }
            }
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

   fn get<R: DeserializeOwned>(&mut self, endpoint: Endpoint, query: &[(&str, &str)]) -> Result<R, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, endpoint);

        let mut request = self.agent.get(&endpoint);
        for &(param, value) in query.iter() {
            request = request.query(param, value);
        }
        request.call()?
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

        self.agent.post(&endpoint)
            .set("Authorization", &format!("Token {}", token))
            .send_json(data)?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Endpoint: `submit-listens`
    pub fn submit_listens(&mut self, token: &str, data: Submission) -> Result<SubmitListensResponse, Error> {
        self.post(Endpoint::SubmitListens, token, data)
    }

    /// Endpoint: `validate-token`
    pub fn validate_token(&mut self, token: &str) -> Result<ValidateTokenResponse, Error> {
        self.get(Endpoint::ValidateToken, &[("token", token)])
    }

    /// Endpoint: `delete-listen`
    pub fn delete_listen(&mut self, token: &str, data: DeleteListen) -> Result<DeleteListenResponse, Error> {
        self.post(Endpoint::DeleteListen, token, data)
    }

    /// Endpoint: `user/{user_name}/listen-count`
    pub fn user_listen_count(&mut self, user: &str) -> Result<UserListenCountResponse, Error> {
        self.get(Endpoint::UserListenCount(user), &[])
    }

    /// Endpoint: `status/get-dump-info`
    pub fn status_get_dump_info(&mut self, id: Option<i64>) -> Result<StatusGetDumpInfoResponse, Error> {
        if let Some(id) = id {
            let id = id.to_string();
            self.get(Endpoint::StatusGetDumpInfo, &[("id", &id)])
        } else {
            self.get(Endpoint::StatusGetDumpInfo, &[])
        }
    }
}
