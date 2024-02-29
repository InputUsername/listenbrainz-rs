use self::raw::Client;

use super::*;

#[test]
fn live_test_retrieve_listens() {
    let listens = Client::new().user_listens("rustynova", None, None, Some(900), None).unwrap();

    assert_eq!(listens.payload.count, 900)
}