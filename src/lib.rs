//! API bindings for ListenBrainz.

mod client;
mod endpoint;
mod error;
pub mod models;

pub use client::Client;
pub use error::Error;
