use listenbrainz::raw::Client;

fn main() {
    let user_name = std::env::args().nth(1).expect("No username provided");

    let client = Client::new();

    let result = client.stats_user_listening_activity(&user_name, None);
    println!("{:#?}", result);

    let result = client.stats_user_daily_activity(&user_name, None);
    println!("{:#?}", result);
}
