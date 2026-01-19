use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AuthSuccessResponse {
    pub token: String,
    pub message: String,
}
