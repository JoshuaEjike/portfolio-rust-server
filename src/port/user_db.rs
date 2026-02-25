use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::{DirectUsersAccess, DirectUsersDetails, Users};
use crate::error::api_error::ApiErrors;
use crate::fields::Email;
use crate::payload_description::user_payload_description::UpdateUserDetails;

#[async_trait]
pub trait UserDBServices: Send + Sync {
    async fn create_user(&self, user: &Users) -> Result<(), ApiErrors>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<DirectUsersAccess>, ApiErrors>;
    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<DirectUsersDetails>, ApiErrors>;
    async fn delete_user(&self, user_id: &Uuid) -> Result<bool, ApiErrors>;
    async fn update_user(&self, users: UpdateUserDetails) -> Result<bool, ApiErrors>;
    async fn find_all_users(&self) -> Result<Vec<DirectUsersDetails>, ApiErrors>;
}
