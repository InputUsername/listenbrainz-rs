use listenbrainz::raw::Client;

fn main() {
    let token = std::env::args().nth(1).expect("No token provided");

    let client = Client::new();

    loop {
        match client.validate_token(&token) {
            Ok(response) => {
                println!("{:?}", response.rate_limit);

                if let Some(remaining) = response.rate_limit.map(|r| r.remaining) {
                    if remaining == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }
    }
}
