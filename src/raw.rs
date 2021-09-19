//! Contains low-level functionality to work with the ListenBrainz API.
//!
//! This module is mostly a thin wrapper around the HTTP API.
//! It contains the [`Client`] type, as well as low-level models
//! for [request] and [response] data.

mod client;
mod endpoint;
pub mod jspf;
pub mod request;
pub mod response;

pub use self::client::Client;
