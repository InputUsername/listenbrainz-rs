use listenbrainz::raw::request::DeleteListen;
use listenbrainz::raw::Client;

fn main() {
    let mut args = std::env::args().skip(1);
    let token = args.next().expect("No token provided");
    let listened_at = args.next().expect("No listened_at provided");
    let recording_msid = args.next().expect("No recording_msid provided");

    let client = Client::new();

    let delete = DeleteListen {
        listened_at: listened_at.parse().unwrap(),
        recording_msid,
    };
    let result = client.delete_listen(&token, delete);
    println!("{:#?}", result);
}
