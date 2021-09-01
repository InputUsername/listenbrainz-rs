use serde::Deserialize;

use attohttpc::Response;

/// Represents errors that can occur while interacting with the API.
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

    /// The request or response data could not be converted into or from JSON.
    #[error("could not convert request or response data into or from JSON")]
    Json(#[source] attohttpc::Error),

    /// There was some other HTTP error while interacting with the API.
    #[error("HTTP error")]
    Http(#[source] attohttpc::Error),

    /// The token that was attempted to be used for authentication is invalid.
    #[error("invalid authentication token")]
    InvalidToken,

    /// Tried to access a service that requires authentication.
    #[error("not authenticated")]
    NotAuthenticated,
}

impl Error {
    /// If the response is a client or server error (status [400-599]),
    /// deserialize it into `Error::Api`. Otherwise, return the original response.
    pub(crate) fn try_from_error_response(response: Response) -> Result<Response, Self> {
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let api_error: ApiError = response.json()?;
            Err(api_error.into())
        } else {
            Ok(response)
        }
    }
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

impl From<attohttpc::Error> for Error {
    fn from(error: attohttpc::Error) -> Self {
        match error.kind() {
            attohttpc::ErrorKind::Json(_) => Self::Json(error),
            _ => Self::Http(error),
        }
    }
}
