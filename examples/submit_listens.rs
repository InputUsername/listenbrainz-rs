use std::time::{SystemTime, UNIX_EPOCH};

use listenbrainz::models::request::{ListenType, Payload, SubmitListens, TrackMetadata};
use listenbrainz::Client;

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn main() {
    let token = std::env::args().nth(1).expect("No token provided");

    let client = Client::new();

    // Submit single

    let listen = Payload {
        listened_at: Some(now()),
        track_metadata: TrackMetadata {
            artist_name: "Rick Astley",
            track_name: "Never Gonna Give You Up",
            release_name: None,
            additional_info: None,
        },
    };
    let submission = SubmitListens {
        listen_type: ListenType::Single,
        payload: &[listen],
    };

    let result = client.submit_listens(&token, submission);
    println!("{:#?}", result);

    // Submit playing now

    let now_playing = Payload {
        listened_at: None,
        track_metadata: TrackMetadata {
            artist_name: "Rick Astley",
            track_name: "Never Gonna Give You Up",
            release_name: None,
            additional_info: None,
        },
    };
    let submission = SubmitListens {
        listen_type: ListenType::PlayingNow,
        payload: &[now_playing],
    };

    let result = client.submit_listens(&token, submission);
    println!("{:#?}", result);

    // Submit imports

    let first = Payload {
        listened_at: Some(now() - 60 * 5),
        track_metadata: TrackMetadata {
            artist_name: "Lil Tecca",
            track_name: "All Star (with Lil Tjay)",
            release_name: None,
            additional_info: None,
        },
    };
    let second = Payload {
        listened_at: Some(now() - 60 * 15),
        track_metadata: TrackMetadata {
            artist_name: "Queens of the Stone Age",
            track_name: "None One Knows",
            release_name: Some("Songs For The Deaf"),
            additional_info: None,
        },
    };
    let third = Payload {
        listened_at: Some(now() - 60 * 10),
        track_metadata: TrackMetadata {
            artist_name: "Alkaline Trio",
            track_name: "Fall Victim",
            release_name: Some("Crimson"),
            additional_info: None,
        },
    };

    let submission = SubmitListens {
        listen_type: ListenType::Import,
        payload: &[first, second, third],
    };

    let result = client.submit_listens(&token, submission);
    println!("{:#?}", result);
}
