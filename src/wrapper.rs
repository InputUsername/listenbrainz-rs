use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error;
use crate::raw::request::{ListenType, Payload, SubmitListens, TrackMetadata};
use crate::raw::Client;

/// Contains a ListenBrainz token and the associated username
/// for authentication purposes.
#[derive(Debug)]
struct Auth {
    token: String,
    user: String,
}

/// An ergonomic ListenBrainz client.
///
/// As opposed to [`Client`], this aims to be a convenient and high-level
/// wrapper of the ListenBrainz API.
#[derive(Debug)]
pub struct ListenBrainz {
    client: Client,
    auth: Option<Auth>,
}

impl ListenBrainz {
    /// Construct a new ListenBrainz client that is not authenticated.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            auth: None,
        }
    }

    /// Construct a new ListenBrainz client with a custom API URL that is not authenticated.
    pub fn new_with_url(url: &str) -> Self {
        Self {
            client: Client::new_with_url(url),
            auth: None,
        }
    }

    /// Get the API URL of this client.
    pub fn api_url(&self) -> &str {
        self.client.api_url()
    }

    /// Check if this client is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.auth.is_some()
    }

    /// Return the token if authenticated or [`None`] if not.
    pub fn authenticated_token(&self) -> Option<&str> {
        self.auth.as_ref().map(|auth| auth.token.as_str())
    }

    /// Return the user if authenticated or [`None`] if not.
    pub fn authenticated_user(&self) -> Option<&str> {
        self.auth.as_ref().map(|auth| auth.user.as_str())
    }

    /// Authenticate this client with the given token.
    /// If the token is valid, authenticates the client.
    /// In case the client was already authenticated, the old information
    /// is discarded and the new token will be used.
    ///
    /// # Errors
    ///
    /// If the token was invalid, returns [`Error::InvalidToken`].
    /// If there was an error while validating the token, that error is returned.
    /// See the Errors section of [`Client`] for more info on what errors might occur.
    pub fn authenticate(&mut self, token: &str) -> Result<(), Error> {
        let result = self.client.validate_token(token)?;
        if result.valid && result.user_name.is_some() {
            self.auth.replace(Auth {
                token: token.to_string(),
                user: result.user_name.unwrap(),
            });
            return Ok(());
        }
        Err(Error::InvalidToken)
    }

    /// Helper method to submit a listen (either "single" or "playing now").
    fn submit_listen(
        &self,
        listen_type: ListenType,
        timestamp: Option<i64>,
        artist: &str,
        track: &str,
        release: Option<&str>,
    ) -> Result<(), Error> {
        let token = self.authenticated_token().ok_or(Error::NotAuthenticated)?;

        let payload = Payload {
            listened_at: timestamp,
            track_metadata: TrackMetadata {
                artist_name: artist,
                track_name: track,
                release_name: release,
                additional_info: None,
            },
        };

        self.client.submit_listens(
            token,
            SubmitListens {
                listen_type,
                payload: &[payload],
            },
        )?;

        Ok(())
    }

    /// Submit a listened track with the current time as the listen time.
    /// This requires authentication.
    ///
    /// # Errors
    ///
    /// If not authenticated, returns [`Error::NotAuthenticated`].
    /// Otherwise, see the Errors section of [`Client`] for more info on
    /// what errors might occur.
    pub fn listen(&self, artist: &str, track: &str, release: Option<&str>) -> Result<(), Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap();

        self.submit_listen(ListenType::Single, Some(now), artist, track, release)
    }

    /// Submit a listened track with the given listen time, intended for importing
    /// previously saved listens. This requires authentication.
    ///
    /// # Errors
    ///
    /// If not authenticated, returns [`Error::NotAuthenticated`].
    /// Otherwise, see the Errors section of [`Client`] for more info on
    /// what errors might occur.
    pub fn import(
        &self,
        artist: &str,
        track: &str,
        release: Option<&str>,
        timestamp: i64,
    ) -> Result<(), Error> {
        self.submit_listen(ListenType::Import, Some(timestamp), artist, track, release)
    }

    /// Submit a currently playing track. This requires authentication.
    ///
    /// # Errors
    ///
    /// If not authenticated, returns [`Error::NotAuthenticated`].
    /// Otherwise, see the Errors section of [`Client`] for more info on
    /// what errors might occur.
    pub fn playing_now(
        &self,
        artist: &str,
        track: &str,
        release: Option<&str>,
    ) -> Result<(), Error> {
        self.submit_listen(ListenType::PlayingNow, None, artist, track, release)
    }
}

impl Default for ListenBrainz {
    fn default() -> Self {
        Self::new()
    }
}
