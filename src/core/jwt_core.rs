use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::user::DirectUsersDetails,
    error::api_error::ApiErrors,
    port::{UserDBServices, jwt::JwtService},
};

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

// pub fn generate_refresh_token() -> String {
//     Uuid::new_v4().to_string()
// }
