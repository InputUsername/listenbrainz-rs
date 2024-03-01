//! API bindings for ListenBrainz.
//!
//! This crate aims to be an idiomatic wrapper of the [ListenBrainz HTTP API] (version 1).
//! It contains functionality for direct access to the API in the [`raw`] module, as well
//! as a more convenient [`ListenBrainz`] client which is easier to use.
//!
//! [ListenBrainz HTTP API]: https://listenbrainz.readthedocs.io/en/production/dev/api/
//!
//! Generally, using the `raw` functionality is more cumbersome, as its types and functions
//! map one-to-one to the HTTP API's JSON input- and response data. Using the `ListenBrainz`
//! type is therefore recommended.
//!
//! # Example
//!
//! Submit a currently playing song to ListenBrainz.org:
//! ```no_run
//! # use listenbrainz::ListenBrainz;
//! #
//! let mut client = ListenBrainz::new();
//!
//! client.authenticate("LISTENBRAINZ TOKEN")
//!     .expect("Could not authenticate with ListenBrainz");
//!
//! client.playing_now("The Beatles", "Here Comes the Sun", Some("Abbey Road"))
//!     .expect("Could not submit 'playing now' request");
//! ```
//!
//! Use a custom API URL, for example to submit songs to [Maloja]:
//! ```no_run
//! # use listenbrainz::ListenBrainz;
//! #
//! let mut client = ListenBrainz::new_with_url("http://maloja.example.com/apis/listenbrainz");
//!
//! client.authenticate("MALOJA API KEY")
//!     .expect("Could not authenticate with Maloja");
//!
//! client.listen("Lymbyc Systym", "Split Stones", None)
//!     .expect("Could not submit listen");
//! ```
//!
//! [Maloja]: https://github.com/krateng/maloja

#![deny(
    missing_docs,
    missing_debug_implementations,
    unsafe_code,
    unstable_features
)]

mod error;
pub mod raw;
mod wrapper;

pub use crate::error::Error;
pub use crate::wrapper::ListenBrainz;


#[cfg(test)]
pub mod tests;