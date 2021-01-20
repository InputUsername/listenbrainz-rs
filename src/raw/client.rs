use serde::de::DeserializeOwned;
use serde::Serialize;

use ureq::Agent;

use super::endpoint::Endpoint;
use super::request::*;
use super::response::*;
use crate::Error;

const API_ROOT_URL: &str = "https://api.listenbrainz.org/1/";

/// Low-level client that directly wraps the ListenBrainz HTTP API.
///
/// Client exposes functions that map one-to-one to the API methods described
/// in the [ListenBrainz API docs](https://listenbrainz.readthedocs.io/en/production/dev/api/).
///
/// # Errors
///
/// Client's methods can return the following errors:
/// - [`Error::Api`]: the API returned a non-`2XX` status.
/// - [`Error::RequestJson`]: the request data could not be converted into JSON.
/// - [`Error::ResponseJson`]: the response data could not be converted into JSON.
/// - [`Error::Http`]: there was some other HTTP error while interacting with the API.
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

    /// Helper method to perform a GET request against an endpoint
    /// without any query parameters.
    fn get<R: DeserializeOwned>(&self, endpoint: Endpoint) -> Result<R, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, endpoint);

        self.agent
            .get(&endpoint)
            .call()?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Helper method to perform a GET request against (most) statistics
    /// endpoints that share common query parameters.
    fn get_stats<R: DeserializeOwned>(
        &self,
        endpoint: Endpoint,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>,
    ) -> Result<Option<R>, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, endpoint);

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

        let response = request.call()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            response.into_json().map(Some).map_err(Error::ResponseJson)
        }
    }

    /// Helper method to perform a POST request against an endpoint
    /// that expects `Serialize`-able input data.
    fn post<D, R>(&self, endpoint: Endpoint, token: &str, data: D) -> Result<R, Error>
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

    /// Endpoint: [`submit-listens`](https://listenbrainz.readthedocs.io/en/production/dev/api/#post--1-submit-listens)
    pub fn submit_listens(
        &self,
        token: &str,
        data: SubmitListens,
    ) -> Result<SubmitListensResponse, Error> {
        self.post(Endpoint::SubmitListens, token, data)
    }

    /// Endpoint: [`validate-token`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-validate-token)
    pub fn validate_token(&self, token: &str) -> Result<ValidateTokenResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::ValidateToken);

        self.agent
            .get(&endpoint)
            .query("token", token)
            .call()?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Endpoint: [`delete-listen`](https://listenbrainz.readthedocs.io/en/production/dev/api/#post--1-delete-listen)
    pub fn delete_listen(
        &self,
        token: &str,
        data: DeleteListen,
    ) -> Result<DeleteListenResponse, Error> {
        self.post(Endpoint::DeleteListen, token, data)
    }

    /// Endpoint: [`users/{user_list}/recent-listens`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-users-(user_list)-recent-listens)
    pub fn users_recent_listens(
        &self,
        user_list: &[&str],
    ) -> Result<UsersRecentListensResponse, Error> {
        self.get(Endpoint::UsersRecentListens(user_list))
    }

    /// Endpoint: [`user/{user_name}/listen-count`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-user-(user_name)-listen-count)
    pub fn user_listen_count(&self, user_name: &str) -> Result<UserListenCountResponse, Error> {
        self.get(Endpoint::UserListenCount(user_name))
    }

    /// Endpoint: [`user/{user_name}/playing-now`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-user-(user_name)-playing-now)
    pub fn user_playing_now(&self, user_name: &str) -> Result<UserPlayingNowResponse, Error> {
        self.get(Endpoint::UserPlayingNow(user_name))
    }

    /// Endpoint: [`user/{user_name}/listens`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-user-(user_name)-listens)
    pub fn user_listens(
        &self,
        user_name: &str,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        count: Option<u64>,
        time_range: Option<u64>,
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

    /// Endpoint: [`latest-import`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-latest-import) (`GET`)
    pub fn get_latest_import(&self, user_name: &str) -> Result<GetLatestImportResponse, Error> {
        let endpoint = format!("{}{}", API_ROOT_URL, Endpoint::LatestImport);

        self.agent
            .get(&endpoint)
            .query("user_name", user_name)
            .call()?
            .into_json()
            .map_err(Error::ResponseJson)
    }

    /// Endpoint: [`latest-import`](https://listenbrainz.readthedocs.io/en/production/dev/api/#post--1-latest-import) (`POST`)
    pub fn update_latest_import(
        &self,
        token: &str,
        data: UpdateLatestImport,
    ) -> Result<UpdateLatestImportResponse, Error> {
        self.post(Endpoint::LatestImport, token, data)
    }

    /// Endpoint: [`stats/sitewide/artists`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-sitewide-artists)
    pub fn stats_sitewide_artists(
        &self,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>,
    ) -> Result<Option<StatsSitewideArtistsResponse>, Error> {
        self.get_stats(Endpoint::StatsSitewideArtists, count, offset, range)
    }

    /// Endpoint: [`stats/user/{user_name}/listening-activity`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-user-(user_name)-listening-activity)
    pub fn stats_user_listening_activity(
        &self,
        user_name: &str,
        range: Option<&str>,
    ) -> Result<Option<StatsUserListeningActivityResponse>, Error> {
        let endpoint = format!(
            "{}{}",
            API_ROOT_URL,
            Endpoint::StatsUserListeningActivity(user_name)
        );

        let mut request = self.agent.get(&endpoint);

        if let Some(range) = range {
            request = request.query("range", range);
        }

        let response = request.call()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            response.into_json().map(Some).map_err(Error::ResponseJson)
        }
    }

    /// Endpoint: [`stats/user/{user_name}/daily-activity`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-user-(user_name)-daily-activity)
    pub fn stats_user_daily_activity(
        &self,
        user_name: &str,
        range: Option<&str>,
    ) -> Result<Option<StatsUserDailyActivityResponse>, Error> {
        let endpoint = format!(
            "{}{}",
            API_ROOT_URL,
            Endpoint::StatsUserDailyActivity(user_name)
        );

        let mut request = self.agent.get(&endpoint);

        if let Some(range) = range {
            request = request.query("range", range);
        }

        let response = request.call()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            response.into_json().map(Some).map_err(Error::ResponseJson)
        }
    }

    /// Endpoint: [`stats/user/{user_name}/recordings`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-user-(user_name)-recordings)
    pub fn stats_user_recordings(
        &self,
        user_name: &str,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>,
    ) -> Result<Option<StatsUserRecordingsResponse>, Error> {
        self.get_stats(
            Endpoint::StatsUserRecordings(user_name),
            count,
            offset,
            range,
        )
    }

    /// Endpoint: [`stats/user/{user_name}/artist-map`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-user-(user_name)-artist-map)
    pub fn stats_user_artist_map(
        &self,
        user_name: &str,
        range: Option<&str>,
        force_recalculate: Option<bool>,
    ) -> Result<StatsUserArtistMapResponse, Error> {
        let endpoint = format!(
            "{}{}",
            API_ROOT_URL,
            Endpoint::StatsUserArtistMap(user_name)
        );

        let mut request = self.agent.get(&endpoint);

        if let Some(range) = range {
            request = request.query("range", range);
        }
        if let Some(force_recalculate) = force_recalculate {
            request = request.query("force_recalculate", &force_recalculate.to_string());
        }

        request.call()?.into_json().map_err(Error::ResponseJson)
    }

    /// Endpoint: [`stats/user/{user_name}/releases`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-user-(user_name)-releases)
    pub fn stats_user_releases(
        &self,
        user_name: &str,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>,
    ) -> Result<Option<StatsUserReleasesResponse>, Error> {
        self.get_stats(Endpoint::StatsUserReleases(user_name), count, offset, range)
    }

    /// Endpoint: [`stats/user/{user_name}/artists`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-stats-user-(user_name)-artists)
    pub fn stats_user_artists(
        &self,
        user_name: &str,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>,
    ) -> Result<Option<StatsUserArtistsResponse>, Error> {
        self.get_stats(Endpoint::StatsUserArtists(user_name), count, offset, range)
    }

    /// Endpoint: [`status/get-dump-info`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-status-get-dump-info)
    pub fn status_get_dump_info(
        &self,
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
