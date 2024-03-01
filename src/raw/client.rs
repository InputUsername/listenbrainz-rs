use attohttpc::header::AUTHORIZATION;
use serde::Serialize;

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
/// - [`Error::Json`]: the request or response data could not be converted from or into JSON.
/// - [`Error::Http`]: there was some other HTTP error while interacting with the API.
#[derive(Debug)]
pub struct Client {
    api_root_url: String,
}

impl Client {
    /// Construct a new client.
    pub fn new() -> Self {
        Self::new_with_url(API_ROOT_URL)
    }

    /// Construct a new client with a custom API URL.
    pub fn new_with_url(url: impl ToString) -> Self {
        Self {
            api_root_url: url.to_string(),
        }
    }

    /// Get the API URL of this client.
    pub fn api_url(&self) -> &str {
        &self.api_root_url
    }

    /// Helper method to perform a GET request against an endpoint
    /// without any query parameters.
    fn get<R: ResponseType>(&self, endpoint: Endpoint) -> Result<R, Error> {
        let endpoint = format!("{}{}", self.api_root_url, endpoint);

        let response = attohttpc::get(endpoint).send()?;

        R::from_response(response)
    }

    /// Helper method to perform a GET request against (most) statistics
    /// endpoints that share common query parameters.
    fn get_stats<R: ResponseType>(
        &self,
        endpoint: Endpoint,
        count: Option<u64>,
        offset: Option<u64>,
        range: Option<&str>,
    ) -> Result<Option<R>, Error> {
        let endpoint = format!("{}{}", self.api_root_url, endpoint);

        let mut request = attohttpc::get(endpoint);

        if let Some(count) = count {
            request = request.param("count", count);
        }
        if let Some(offset) = offset {
            request = request.param("offset", offset);
        }
        if let Some(range) = range {
            request = request.param("range", range);
        }

        let response = request.send()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            R::from_response(response).map(Some)
        }
    }

    /// Helper method to perform a POST request against an endpoint
    /// that expects `Serialize`-able input data.
    fn post<D, R>(&self, endpoint: Endpoint, token: &str, data: D) -> Result<R, Error>
    where
        D: Serialize,
        R: ResponseType,
    {
        let endpoint = format!("{}{}", self.api_root_url, endpoint);

        let response = attohttpc::post(endpoint)
            .header(AUTHORIZATION, format!("Token {token}"))
            .json(&data)?
            .send()?;

        R::from_response(response)
    }

    /// Endpoint: [`submit-listens`](https://listenbrainz.readthedocs.io/en/production/dev/api/#post--1-submit-listens)
    pub fn submit_listens<Track: StrType, Artist: StrType, Release: StrType>(
        &self,
        token: &str,
        data: SubmitListens<Track, Artist, Release>,
    ) -> Result<SubmitListensResponse, Error> {
        self.post(Endpoint::SubmitListens, token, data)
    }

    /// Endpoint: [`validate-token`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-validate-token)
    pub fn validate_token(&self, token: &str) -> Result<ValidateTokenResponse, Error> {
        let endpoint = format!("{}{}", self.api_root_url, Endpoint::ValidateToken);

        let response = attohttpc::get(endpoint)
            .header(AUTHORIZATION, format!("Token {token}"))
            .send()?;

        ResponseType::from_response(response)
    }

    /// Endpoint: [`delete-listen`](https://listenbrainz.readthedocs.io/en/production/dev/api/#post--1-delete-listen)
    pub fn delete_listen<T: StrType>(
        &self,
        token: &str,
        data: DeleteListen<T>,
    ) -> Result<DeleteListenResponse, Error> {
        self.post(Endpoint::DeleteListen, token, data)
    }

    /// Endpoint: [`user/{user_name}/listen-count`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-user-(user_name)-listen-count)
    pub fn user_listen_count(&self, user_name: &str) -> Result<UserListenCountResponse, Error> {
        self.get(Endpoint::UserListenCount(user_name))
    }

    /// Endpoint: [`user/{user_name}/playing-now`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-user-(user_name)-playing-now)
    pub fn user_playing_now(&self, user_name: &str) -> Result<UserPlayingNowResponse, Error> {
        self.get(Endpoint::UserPlayingNow(user_name))
    }

    /// Endpoint: [`user/{user_name}/listens`](https://listenbrainz.readthedocs.io/en/latest/users/api/core.html#get--1-user-(user_name)-listens)
    pub fn user_listens(
        &self,
        user_name: &str,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
        count: Option<u64>,
    ) -> Result<UserListensResponse, Error> {
        let endpoint = format!("{}{}", self.api_root_url, Endpoint::UserListens(user_name));

        let mut request = attohttpc::get(endpoint);

        if let Some(min_ts) = min_ts {
            request = request.param("min_ts", min_ts);
        }
        
        if let Some(max_ts) = max_ts {
            request = request.param("max_ts", max_ts);
        }

        if let Some(count) = count {
            request = request.param("count", count);
        }

        let response = request.send()?;

        ResponseType::from_response(response)
    }

    /// Endpoint: [`latest-import`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-latest-import) (`GET`)
    pub fn get_latest_import(&self, user_name: &str) -> Result<GetLatestImportResponse, Error> {
        let endpoint = format!("{}{}", self.api_root_url, Endpoint::LatestImport);

        let response = attohttpc::get(endpoint)
            .param("user_name", user_name)
            .send()?;

        ResponseType::from_response(response)
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

        let mut request = attohttpc::get(endpoint);

        if let Some(range) = range {
            request = request.param("range", range);
        }

        let response = request.send()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            ResponseType::from_response(response).map(Some)
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

        let mut request = attohttpc::get(endpoint);

        if let Some(range) = range {
            request = request.param("range", range);
        }

        let response = request.send()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            ResponseType::from_response(response).map(Some)
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
    ) -> Result<Option<StatsUserArtistMapResponse>, Error> {
        let endpoint = format!(
            "{}{}",
            API_ROOT_URL,
            Endpoint::StatsUserArtistMap(user_name)
        );

        let mut request = attohttpc::get(endpoint);

        if let Some(range) = range {
            request = request.param("range", range);
        }
        if let Some(force_recalculate) = force_recalculate {
            request = request.param("force_recalculate", force_recalculate);
        }

        let response = request.send()?;

        // API returns 204 and an empty document if there are no statistics
        if response.status() == 204 {
            Ok(None)
        } else {
            ResponseType::from_response(response).map(Some)
        }
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

    /// Endpoint: [`GET /1/stats/release-group/(release_group_mbid)/listeners`](https://listenbrainz.readthedocs.io/en/latest/users/api/statistics.html#get--1-stats-release-group-(release_group_mbid)-listeners)
    /// Get the top listeners for a release group, as well as getting the total number of listens for it
    pub fn stats_release_group_listeners(
        &self,
        release_group_mbid: &str,
        range: Option<&str>,
    ) -> Result<Option<StatsReleaseGroupListenersResponse>, Error> {
        self.get_stats(
            Endpoint::StatsReleaseGroupListeners(release_group_mbid),
            None,
            None,
            range,
        )
    }

    /// Endpoint: [`status/get-dump-info`](https://listenbrainz.readthedocs.io/en/production/dev/api/#get--1-status-get-dump-info)
    pub fn status_get_dump_info(
        &self,
        id: Option<i64>,
    ) -> Result<StatusGetDumpInfoResponse, Error> {
        let endpoint = format!("{}{}", self.api_root_url, Endpoint::StatusGetDumpInfo);

        let mut request = attohttpc::get(endpoint);

        if let Some(id) = id {
            request = request.param("id", id);
        }

        let response = request.send()?;

        ResponseType::from_response(response)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
