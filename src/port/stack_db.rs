use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::stack::{DirectStackDetails, Stack},
    error::api_error::ApiErrors,
    payload_description::UpdateStack,
};

#[async_trait]
pub trait StackDBServices: Send + Sync {
    async fn create_stack(&self, stack: &Stack) -> Result<Uuid, ApiErrors>;
    async fn find_by_title(&self, title: &str) -> Result<Option<DirectStackDetails>, ApiErrors>;
    async fn find_by_id(&self, stack_id: &Uuid) -> Result<Option<DirectStackDetails>, ApiErrors>;
    async fn delete_stack(&self, stack_id: &Uuid) -> Result<bool, ApiErrors>;
    async fn update_stack(&self, stack: UpdateStack) -> Result<bool, ApiErrors>;
    async fn find_all_stack(&self) -> Result<Vec<DirectStackDetails>, ApiErrors>;
}
