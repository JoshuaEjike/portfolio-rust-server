use std::sync::Arc;

use crate::{
    core::jwt_core::{generate_refresh_token, generate_token},
    domain::refresh_token::TokenPair,
    error::api_error::ApiErrors,
    port::refresh_token_db::RefreshTokenDBServices,
};

pub struct RefreshTokenServices {
    repo: Arc<dyn RefreshTokenDBServices + Send + Sync>,
    secret: String,
    expiry_second: u64,
}

impl RefreshTokenServices {
    pub fn new(
        repo: Arc<dyn RefreshTokenDBServices + Send + Sync>,
        secret: String,
        expiry_second: u64,
    ) -> Self {
        Self {
            repo,
            secret,
            expiry_second,
        }
    }

    pub async fn handle_login(&self, user_id: uuid::Uuid) -> Result<TokenPair, ApiErrors> {
        let access_token = generate_token(user_id, &self.secret, self.expiry_second)?;

        let refresh_token = generate_refresh_token();

        self.repo
            .store_refresh_token(user_id, &refresh_token)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub async fn handle_refresh(&self, token: String) -> Result<TokenPair, ApiErrors> {
        let record = self
            .repo
            .find_refresh_token(&token)
            .await?
            .ok_or(ApiErrors::Unauthorized("Invalid refresh token".into()))?;

        if record.revoked.unwrap_or(false) {
            return Err(ApiErrors::Unauthorized("Token revoked".into()));
        }

        if record.expires_at < chrono::Utc::now().naive_utc() {
            return Err(ApiErrors::Unauthorized("Token expired".into()));
        }

        // 🔁 ROTATION
        self.repo.revoke_refresh_token(record.id).await?;

        let new_refresh = generate_refresh_token();

        self.repo
            .store_refresh_token(record.user_id, &new_refresh)
            .await?;

        let access = generate_token(record.user_id, &self.secret, self.expiry_second)?;

        Ok(TokenPair {
            access_token: access,
            refresh_token: new_refresh,
        })
    }

    pub async fn handle_logout(&self, token: String) -> Result<(), ApiErrors> {
        self.repo.revoke_refresh_token_by_value(&token).await?;

        Ok(())
    }
}
