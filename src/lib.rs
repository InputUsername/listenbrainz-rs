//! API bindings for ListenBrainz.

mod client;
mod endpoint;
mod error;
pub mod models;

pub use crate::client::Client;
pub use crate::error::Error;
