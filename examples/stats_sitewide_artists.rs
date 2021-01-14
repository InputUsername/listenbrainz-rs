use listenbrainz::Client;

fn main() {
    let mut client = Client::new();

    let result = client.stats_sitewide_artists(None, None, Some("year"));
    println!("{:#?}", result);
}
