use listenbrainz::raw::Client;

fn main() {
    let client = Client::new();

    let playlist = client
        .get_playlist("f316bb0e-8e26-44f8-a802-6d2a3688fc7d")
        .unwrap();
    println!("Playlist: {:#?}", playlist)
}
