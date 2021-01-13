use serde::de::DeserializeOwned;
use serde::Serialize;

use ureq::Agent;

use crate::endpoint::Endpoint;
use crate::Error;
use crate::models::request::*;
use crate::models::response::*;

const API_ROOT_URL: &str = "https://api.listenbrainz.org/1/";

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

    /// Endpoint: `user/{user_name}/playing-now`
    pub fn user_playing_now(&mut self, user_name: &str) -> Result<UserPlayingNowResponse, Error> {
        self.get(Endpoint::UserPlayingNow(user_name))
    }

    /// Endpoint: `user/{user_name}/listens`
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

    /// Endpoint: `stats/sitewide/artists`
    pub fn stats_sitewide_artists(
        &mut self,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>
    ) -> Result<StatsSitewideArtistsResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::StatsSitewideArtists);

        let mut request = self.agent.get(&endpoint);

        if let Some(count) = count {
            request = request.query("count", &count.to_string());
        }
        if let Some(offset) = offset {
            request = request.query("offset", &offset.to_string());
        }
        if let Some(range) = range {
            request = request.query("range", range);
        }

        request.call()?.into_json().map_err(Error::ResponseJson)
    }

    // /// Endpoint: `stats/user/{user_name}/listening-activity`

    // /// Endpoint: `stats/user/{user_name}/daily-activity`

    // /// Endpoint: `stats/user/{user_name}/recordings`

    // /// Endpoint: `stats/user/{user_name}/artist-map`

    // /// Endpoint: `stats/user/{user_name}/releases`

    // /// Endpoint: `stats/user/{user_name}/artists`

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
