use axum::{extract::FromRequestParts, http::request::Parts};

use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use uuid::Uuid;

use crate::{
    core::jwt_core::validate_user_token,
    error::api_error::ApiErrors,
    fields::{Email, Roles, Text},
    state::AppState,
};

pub struct AuthUser {
    pub id: Uuid,
    pub email: Email,
    pub name: Text,
    pub roles: Roles,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiErrors;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| ApiErrors::Unauthorized("Missing token".into()))?;

        let token = bearer.token();

        let user =
            validate_user_token(token, state.jwt_services.clone(), state.user_repo.clone()).await?;

        Ok(AuthUser {
            id: user.id,
            email: user.email,
            name: user.name,
            roles: user.roles,
        })
    }
}
