use listenbrainz::Client;

fn main() {
    let token = std::env::args().nth(1).expect("No token provided");

    let client = Client::new();

    let result = client.validate_token(&token);
    println!("{:#?}", result);
}
