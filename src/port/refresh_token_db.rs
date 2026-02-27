use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::refresh_token::RefreshTokenRecord, error::api_error::ApiErrors};

#[async_trait]
pub trait RefreshTokenDBServices: Send + Sync {
    async fn store_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), ApiErrors>;

    async fn find_refresh_token(
        &self,
        token: &str,
    ) -> Result<Option<RefreshTokenRecord>, ApiErrors>;

    async fn revoke_refresh_token(&self, id: Uuid) -> Result<(), ApiErrors>;

    async fn revoke_refresh_token_by_value(&self, token: &str) -> Result<(), ApiErrors>;
}
