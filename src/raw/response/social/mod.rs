use serde::Deserialize;
use serde::Serialize;

use crate::raw::response::response_type;

// --------- GET /1/user/(user_name)/followers
// https://listenbrainz.readthedocs.io/en/latest/users/api/social.html#get--1-user-(user_name)-followers

response_type! {
    /// Response type for [`Client::user_followers`](super::Client::user_followers).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserFollowersResponse {
        pub followers: Vec<String>,
        pub user: String,
    }
}

// --------- GET /1/user/(user_name)/following
// https://listenbrainz.readthedocs.io/en/latest/users/api/social.html#get--1-user-(user_name)-following

response_type! {
    /// Response type for [`Client::user_following`](super::Client::user_following).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserFollowingResponse {
        pub following: Vec<String>,
        pub user: String,
    }
}

// --------- POST /1/user/(user_name)/follow
// https://listenbrainz.readthedocs.io/en/latest/users/api/social.html#post--1-user-(user_name)-follow

response_type! {
    /// Response type for [`Client::user_follow`](super::Client::user_follow).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserFollowResponse {
        pub status: String,
    }
}

// --------- POST /1/user/(user_name)/unfollow
// https://listenbrainz.readthedocs.io/en/latest/users/api/social.html#post--1-user-(user_name)-unfollow

response_type! {
    /// Response type for [`Client::user_unfollow`](super::Client::user_unfollow).
    #[derive(Debug, Deserialize, Serialize)]
    pub struct UserUnfollowResponse {
        pub status: String,
    }
}
