use serde::Serialize;

use crate::domain::user::DirectUsersDetails;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AuthSuccessResponse {
    pub token: String,
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingUsersPayload {
    pub message: String,
    pub users: Vec<DirectUsersDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingSingleUsersPayload {
    pub message: String,
    pub user: Option<DirectUsersDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SuccessMessageResponse {
    pub message: String,
}
