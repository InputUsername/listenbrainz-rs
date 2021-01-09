use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    code: u16,
    message: String,
}

// --------- submit-listens

#[derive(Debug, Deserialize)]
pub struct SubmitListensResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,
}

// --------- validate-token

#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub valid: bool,
    pub user_name: Option<String>,
}

// --------- delete-listen

#[derive(Debug, Deserialize)]
pub struct DeleteListenResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,
}

// --------- user/{user_name}/listen-count

#[derive(Debug, Deserialize)]
pub struct UserListenCountResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub count: u64,
}

// --------- status/get-dump-info

#[derive(Debug, Deserialize)]
pub struct StatusGetDumpInfoResponse {
    #[serde(flatten)]
    pub api_response: ApiResponse,

    pub id: i64,
    pub timestamp: String,
}
