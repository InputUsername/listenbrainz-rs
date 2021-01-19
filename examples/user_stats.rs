use listenbrainz::raw::Client;

fn main() {
    let mut args = std::env::args().skip(1);
    let subject = args.next().expect("No subject provided");
    let user_name = args.next().expect("No username provided");

    let client = Client::new();

    match subject.as_str() {
        "recordings" => {
            let result = client.stats_user_recordings(&user_name, None, None, None);
            println!("{:#?}", result);
        }
        "releases" => {
            let result = client.stats_user_releases(&user_name, None, None, None);
            println!("{:#?}", result);
        }
        "artists" => {
            let result = client.stats_user_artists(&user_name, None, None, None);
            println!("{:#?}", result);
        }
        _ => {
            eprintln!(r"Subject must be one of recordings, releases, artists");
        }
    }
}
