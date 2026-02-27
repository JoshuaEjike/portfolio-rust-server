use async_trait::async_trait;
use uuid::Uuid;

use crate::error::api_error::ApiErrors;

#[async_trait]
pub trait JwtService: Send + Sync {
    fn generate(&self, user_id: Uuid) -> Result<String, ApiErrors>;
    fn verify(&self, token: &str) -> Option<String>;
}
