//! Low-level response data models.
//!
//! Every response type has the `rate_limit` field, which contains rate limiting
//! information. See the documentation of the [`RateLimit`] type for more
//! details.

#![allow(missing_docs)]

use attohttpc::Response;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::Error;

// Sub modules
pub mod art;
pub mod core;
pub mod metadata;
pub mod playlists;
pub mod popularity;
pub mod recommendations;
pub mod recordings;
pub mod social;
pub mod statistics;

// Reexport of the sub modules
pub use crate::raw::response::art::*;
pub use crate::raw::response::core::*;
pub use crate::raw::response::metadata::*;
pub use crate::raw::response::playlists::*;
pub use crate::raw::response::popularity::*;
pub use crate::raw::response::recommendations::*;
pub use crate::raw::response::recordings::*;
pub use crate::raw::response::social::*;
pub use crate::raw::response::statistics::*;

/// Contains rate limiting information.
///
/// ListenBrainz API rate limiting is described in the [API docs].
/// Prefer using the [`RateLimit::reset_in`] field over [`RateLimit::reset`],
/// as the former is resilient against clients with incorrect clocks.
///
/// [API docs]: https://listenbrainz.readthedocs.io/en/production/dev/api/#rate-limiting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimit {
    pub limit: u64,
    pub remaining: u64,
    pub reset_in: u64,
    pub reset: i64,
}

impl RateLimit {
    /// Extract rate limiting information from the `X-RateLimit-` headers.
    /// Only returns `Some` if all fields are present and valid.
    fn from_headers(response: &Response) -> Option<Self> {
        let headers = response.headers();

        let limit = headers
            .get("X-RateLimit-Limit")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;
        let remaining = headers
            .get("X-RateLimit-Remaining")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;
        let reset_in = headers
            .get("X-RateLimit-Reset-In")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;
        let reset = headers
            .get("X-RateLimit-Reset")?
            .to_str()
            .ok()?
            .parse()
            .ok()?;

        Some(Self {
            limit,
            remaining,
            reset_in,
            reset,
        })
    }
}

/// Internal trait for response types.
/// Allows converting the response type from an `attohttpc::Response`,
/// by deserializing the body into the response type and then
/// adding the `rate_limit` field from headers.
pub(crate) trait ResponseType: DeserializeOwned {
    fn from_response(response: Response) -> Result<Self, Error>;
}

/// Internal macro for response types.
/// Wraps the definition of a response type, adds the `rate_limit` field,
/// and implements the `ResponseType` trait.
macro_rules! response_type {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field:ident: $field_ty:ty),*
            $(,)?
        }
    ) => {
        $(#[$meta])*
        pub struct $name {
            #[serde(skip)]
            pub rate_limit: Option<crate::raw::response::RateLimit>,
            $(pub $field: $field_ty),*
        }

        impl crate::raw::response::ResponseType for $name {
            fn from_response(response: crate::raw::response::Response) -> Result<Self, crate::raw::response::Error> {
                let response = crate::raw::response::Error::try_from_error_response(response)?;
                let rate_limit = crate::raw::response::RateLimit::from_headers(&response);
                let mut result: Self = response.json()?;
                result.rate_limit = rate_limit;
                Ok(result)
            }
        }
    }
}

// Let the childrens access the macro
pub(super) use response_type;

// --------- status/get-dump-info

response_type! {
    /// Response type for [`Client::status_get_dump_info`](super::Client::status_get_dump_info).
    #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
    pub struct StatusGetDumpInfoResponse {
        pub code: u16,
        pub message: String,

        pub id: i64,
        pub timestamp: String,
    }
}
