use async_trait::async_trait;

use crate::error::AuthError;

use crate::domain::user::{Email, UserId, Users};
use crate::payload_description::UpdateUser;

#[async_trait]
pub trait UserDBServices: Send + Sync {
    async fn create_user(&self, user: &Users) -> Result<(), AuthError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<Users>, AuthError>;
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<Users>, AuthError>;
    async fn delete_user(&self, user_id: &UserId) -> Result<bool, AuthError>;
    async fn update_user(&self, users: UpdateUser) -> Result<bool, AuthError>;
    async fn find_all_users(&self) -> Result<Vec<Users>, AuthError>;
}
