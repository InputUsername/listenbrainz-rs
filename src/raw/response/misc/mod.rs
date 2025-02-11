use serde::Deserialize;
use serde::Serialize;

use crate::raw::response::response_type;

// ---------  GET /1/status/get-dump-info
// https://listenbrainz.readthedocs.io/en/latest/users/api/misc.html#get--1-status-get-dump-info

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
