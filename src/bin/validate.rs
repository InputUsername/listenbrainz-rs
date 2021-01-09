use listenbrainz::Client;

fn main() {
    let token = std::env::args().nth(1).unwrap();
    let mut client = Client::new();
    println!("{:#?}", client.validate_token(&token));
}
