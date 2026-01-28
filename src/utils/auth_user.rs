use crate::{
    domain::{user::Users, uuid_lib::Id},
    error::AuthError,
    port::{UserDBServices, jwt::JwtService},
};

use axum::http::HeaderMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CurrentUser(pub Users);

impl CurrentUser {
    pub async fn from_headers(
        headers: &HeaderMap,
        jwt_service: Arc<dyn JwtService>,
        user_repo: Arc<dyn UserDBServices>,
    ) -> Result<Self, AuthError> {
        // 🔐 Extract Authorization header
        let auth_header = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AuthError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::Unauthorized)?;

        // ✅ Verify token → get user_id as String
        let user_id_str = jwt_service.verify(token).ok_or(AuthError::Unauthorized)?; // Option<String> → String

        // ✅ Convert into Id (your domain type)
        let user_id = Id::from_str(&user_id_str).map_err(AuthError::InvalidString)?;
        // ^ make sure you have a constructor for this (e.g. new(String))

        // ✅ Look up user in database
        let user = user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or(AuthError::Unauthorized)?;

        Ok(CurrentUser(user))
    }
}
