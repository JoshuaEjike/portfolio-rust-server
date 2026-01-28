use async_trait::async_trait;

use crate::{
    domain::{
        stack::{DirectStackDetails, Stack},
        uuid_lib::Id,
    },
    error::stack_error::StackError,
    payload_description::UpdateStack,
};

#[async_trait]
pub trait StackDBServices: Send + Sync {
    async fn create_stack(&self, stack: &Stack) -> Result<(), StackError>;
    async fn find_by_title(&self, title: &str) -> Result<Option<Stack>, StackError>;
    async fn find_by_id(&self, stack_id: &Id) -> Result<Option<DirectStackDetails>, StackError>;
    async fn delete_stack(&self, stack_id: &Id) -> Result<bool, StackError>;
    async fn update_stack(&self, stack: UpdateStack) -> Result<bool, StackError>;
    async fn find_all_stack(&self) -> Result<Vec<DirectStackDetails>, StackError>;
}
