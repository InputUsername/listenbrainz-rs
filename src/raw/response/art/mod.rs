use serde::Deserialize;
use serde::Serialize;

// --------- POST /1/art/grid/
// https://listenbrainz.readthedocs.io/en/latest/users/api/art.html#post--1-art-grid-

/// Response type for [`Client::user_similar_users`](super::Client::user_similar_users).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ArtGridResponse {
    pub rate_limit: Option<crate::raw::response::RateLimit>,
    pub image: String,
}

// This response is special as it return a SVG directly. So we implement `ResponseType` manually
impl crate::raw::response::ResponseType for ArtGridResponse {
    fn from_response(
        response: crate::raw::response::Response,
    ) -> Result<Self, crate::raw::response::Error> {
        let response = crate::raw::response::Error::try_from_error_response(response)?;
        let rate_limit = crate::raw::response::RateLimit::from_headers(&response);
        let image = response.text()?;
        Ok(Self { image, rate_limit })
    }
}
