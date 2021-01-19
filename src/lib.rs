//! API bindings for ListenBrainz.
//!
//! This crate aims to be an idiomatic wrapper of the [ListenBrainz HTTP API] (version 1).
//! It contains functionality for direct access to the API in the [`raw`] module, as well
//! as a more convenient [`ListenBrainz`] client which is easier to use.
//!
//! Generally, using the `raw` functionality is more cumbersome, as its types and functions
//! map one-to-one to the HTTP API's JSON input- and response data. Using the `ListenBrainz`
//! type is therefore recommended.
//!
//! [ListenBrainz HTTP API]: https://listenbrainz.readthedocs.io/en/production/dev/api/

mod error;
pub mod raw;
mod wrapper;

pub use crate::error::Error;
pub use crate::wrapper::ListenBrainz;
