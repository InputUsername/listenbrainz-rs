use listenbrainz::raw::Client;

fn main() {
    let user_name = std::env::args().nth(1).expect("No username provided");

    let client = Client::new();

    let result = client.user_listen_count(&user_name);
    println!("Listen count: {:#?}", result);

    let result = client.user_playing_now(&user_name);
    println!("Playing now: {:#?}", result);

    let result = client.user_listens(&user_name, None, None, Some(5));
    println!("Recent listens: {:#?}", result);
}
