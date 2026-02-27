use jsonwebtoken::{EncodingKey, Header, encode};
use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{
    adapter::jwt_adapter::Claims,
    domain::user::DirectUsersDetails,
    error::api_error::ApiErrors,
    port::{UserDBServices, jwt::JwtService},
};

pub fn generate_token(
    user_id: Uuid,
    jwt_secret: &str,
    expiry_second: u64,
) -> Result<String, ApiErrors> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(expiry_second as i64))
        .expect("valid timespame")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiErrors::InternalServerError("Token generation failed".to_string()))
}

pub async fn validate_user_token(
    token: &str,
    jwt_service: Arc<dyn JwtService>,
    user_repo: Arc<dyn UserDBServices>,
) -> Result<DirectUsersDetails, ApiErrors> {
    let user_id_str = jwt_service
        .verify(token)
        .ok_or(ApiErrors::Unauthorized("Invalid or expired token".into()))?;

    let user_id =
        Uuid::parse_str(&user_id_str).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let user = user_repo
        .find_by_id(&user_id)
        .await?
        .ok_or(ApiErrors::NotFound("User not found".to_string()))?;

    Ok(user)
}

pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}
