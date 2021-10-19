use std::convert::TryInto;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error;
use crate::raw::request::{ListenType, Payload, SubmitListens, TrackMetadata};
use crate::raw::Client;

mod private {
    pub trait Sealed {}
    impl Sealed for super::NotAuthenticated {}
    impl Sealed for super::Authenticated {}
}

/// A marker trait for authentication status.
///
/// It is used with the "typestate pattern", to enforce authentication of a [`ListenBrainz`] client
/// at compile-time. For more information, see the documentation for the [`ListenBrainz`] type.
///
/// This is a [sealed trait], meaning you cannot implement it for your own types.
///
/// [sealed trait]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub trait AuthStatus: private::Sealed {}

/// The [status](AuthStatus) of an unauthenticated client.
pub struct NotAuthenticated;

/// The [status](AuthStatus) of an authenticated client.
pub struct Authenticated {
    token: String,
}

impl AuthStatus for NotAuthenticated {}
impl AuthStatus for Authenticated {}

/// An ergonomic ListenBrainz client.
///
/// As opposed to [`Client`](crate::raw::Client), this aims to be a convenient and high-level
/// wrapper of the ListenBrainz API.
///
/// This type uses a pattern commonly called the "[typestate pattern]". The `ListenBrainz` struct is
/// generic over its authentication status ([`NotAuthenticated`] or [`Authenticated`]).
/// Methods that require authentication are only available for `ListenBrainz<Authenticated>`.
/// In this way, authentication is enforced at compile-time.
///
/// [typestate pattern]: http://cliffle.com/blog/rust-typestate/
pub struct ListenBrainz<A: AuthStatus = NotAuthenticated> {
    client: Client,
    auth: A,
}

impl ListenBrainz {
    /// Construct a new ListenBrainz client that is not authenticated.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            auth: NotAuthenticated,
        }
    }

    /// Construct a new ListenBrainz client that is not authenticated using a custom API URL.
    pub fn new_with_url(url: &str) -> Self {
        Self {
            client: Client::new_with_url(url),
            auth: NotAuthenticated,
        }
    }

    /// Attempt to authenticate this client with the given user token.
    /// This consumes the client if successful, returning an authenticated client.
    /// If unsuccessful, returns the original (unauthenticated) client.
    ///
    /// # Errors
    ///
    /// If the token was invalid, returns [`Error::InvalidToken`].
    /// If there was an error while validating the token, that error is returned.
    /// See the Errors section of [`Client`] for more info on what errors might occur.
    pub fn authenticate(self, token: &str) -> Result<ListenBrainz<Authenticated>, (Self, Error)> {
        let response = match self.client.validate_token(token) {
            Ok(response) => response,
            Err(err) => return Err((self, err)),
        };

        if response.valid {
            Ok(ListenBrainz {
                client: self.client,
                auth: Authenticated {
                    token: token.to_string()
                },
            })
        } else {
            Err((self, Error::InvalidToken))
        }
    }
}

impl ListenBrainz<Authenticated> {
    pub fn token(&self) -> &str {
        &self.auth.token
    }

    /// Helper method to submit a listen (either "single" or "playing now").
    fn submit_listen(
        &self,
        listen_type: ListenType,
        timestamp: Option<i64>,
        artist: &str,
        track: &str,
        release: &str,
    ) -> Result<(), Error> {
        let payload = Payload {
            listened_at: timestamp,
            track_metadata: TrackMetadata {
                artist_name: artist,
                track_name: track,
                release_name: Some(release),
                additional_info: None,
            },
        };

        self.client.submit_listens(
            &self.auth.token,
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
    /// See the Errors section of [`Client`] for more info on what errors might occur.
    pub fn listen(&self, artist: &str, track: &str, release: &str) -> Result<(), Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Unix epoch is in the future")
            .as_secs()
            .try_into()
            .unwrap();

        self.submit_listen(ListenType::Single, Some(now), artist, track, release)
    }

    /// Submit a currently playing track. This requires authentication.
    ///
    /// # Errors
    ///
    /// See the Errors section of [`Client`] for more info on what errors might occur.
    pub fn playing_now(&self, artist: &str, track: &str, release: &str) -> Result<(), Error> {
        self.submit_listen(ListenType::PlayingNow, None, artist, track, release)
    }
}
