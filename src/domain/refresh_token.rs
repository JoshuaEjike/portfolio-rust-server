use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct RefreshTokenRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: NaiveDateTime,
    pub revoked: Option<bool>,
}
