use std::collections::HashMap;

use serde::Serialize;

// --------- submit-listens

#[derive(Debug, Serialize)]
pub struct Submission<'a> {
    pub listen_type: ListenType,
    pub payload: Vec<Payload<'a>>,
}

#[derive(Debug, Serialize)]
pub enum ListenType {
    Single,
    PlayingNow,
    Import,
}

#[derive(Debug, Serialize)]
pub struct Payload<'a> {
    pub listened_at: i64,
    pub track_metadata: TrackMetadata<'a>
}

#[derive(Debug, Serialize)]
pub struct TrackMetadata<'a> {
    pub artist_name: &'a str,
    pub track_name: &'a str,
    pub release_name: Option<&'a str>,
    pub additional_info: Option<HashMap<&'a str, &'a str>>,
}

// --------- delete-listen

#[derive(Debug, Serialize)]
pub struct DeleteListen<'a> {
    pub listened_at: i64,
    pub recording_msid: &'a str,
}
