use listenbrainz::raw::Client;

fn main() {
    let client = Client::new();
    let playlist = std::env::args().nth(1).expect("No playlist provided");

    // Example playlist: f316bb0e-8e26-44f8-a802-6d2a3688fc7d
    let playlist = client.get_playlist(&playlist).unwrap();
    println!("Playlist: {:#?}", playlist)
}
