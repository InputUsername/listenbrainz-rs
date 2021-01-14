use listenbrainz::Client;

fn main() {
    let users: Vec<String> = std::env::args().skip(1).collect();
    let users_ref: Vec<&str> = users.iter().map(String::as_str).collect();

    let client = Client::new();

    let result = client.users_recent_listens(&users_ref);
    println!("{:#?}", result);
}
