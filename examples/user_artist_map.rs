use listenbrainz::Client;

fn main() {
    let user_name = std::env::args().nth(1).expect("No username provided");

    let client = Client::new();

    let result = client.stats_user_artist_map(&user_name, None, None);
    println!("{:#?}", result);
}
