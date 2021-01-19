use std::io;

use serde::Deserialize;

/// Represents errors that can occor while interacting with the API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The API returned a non-200 status code.
    #[error("API error ({code}): {error}")]
    Api {
        /// The HTTP status code.
        code: u16,

        /// A message describing the error.
        error: String,
    },

    /// The input data could not be converted into JSON.
    #[error("could not convert request input into JSON")]
    RequestJson(#[source] serde_json::Error),

    /// The HTTP response could not be converted into JSON.
    #[error("could not convert HTTP response into JSON")]
    ResponseJson(#[source] io::Error),

    /// There was some other HTTP error while interacting with the API.
    #[error("HTTP error")]
    Http(#[source] Box<ureq::Error>),

    /// Tried to access a service that requires authentication.
    #[error("not authenticated")]
    NotAuthenticated,
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
            ureq::Error::Transport(_) => Error::Http(Box::new(error)),
        }
    }
}
