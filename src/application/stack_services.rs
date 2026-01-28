use std::sync::Arc;

use crate::{
    domain::{
        stack::{DirectStackDetails, Stack},
        uuid_lib::Id,
    },
    error::stack_error::StackError,
    payload_description::{CreateStackData, UpdateStack},
    port::StackDBServices,
};

pub struct StackServices {
    repo: Arc<dyn StackDBServices + Send + Sync>,
}

impl StackServices {
    pub fn new(repo: Arc<dyn StackDBServices + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_stacks(&self, data: CreateStackData) -> Result<String, StackError> {
        if self.repo.find_by_title(&data.title).await?.is_some() {
            return Err(StackError::StackExists);
        }

        let stack = Stack::new(data)?;

        self.repo.create_stack(&stack).await?;

        Ok("success".to_string())
    }

    pub async fn get_all_stack(&self) -> Result<Vec<DirectStackDetails>, StackError> {
        let stack_data = self.repo.find_all_stack().await?;

        Ok(stack_data)
    }

    pub async fn find_single_stack(
        &self,
        stack_id: &Id,
    ) -> Result<Option<DirectStackDetails>, StackError> {
        let user = self
            .repo
            .find_by_id(stack_id)
            .await?
            .ok_or(StackError::StackNotFound)?;

        Ok(Some(DirectStackDetails {
            id: user.id,
            title: user.title,
            slug: user.slug,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }))
    }

    pub async fn delete_stack(&self, stack_id: &Id) -> Result<bool, StackError> {
        let stack_data = self.repo.delete_stack(stack_id).await?;

        if !stack_data {
            return Err(StackError::StackNotFound);
        }

        Ok(true)
    }

    pub async fn update_stack(&self, stack: UpdateStack) -> Result<bool, StackError> {
        if self.repo.find_by_id(&stack.id).await?.is_none() {
            return Err(StackError::StackNotFound);
        }

        let user_data = self.repo.update_stack(stack).await?;

        if !user_data {
            return Err(StackError::StackNotFound);
        }

        Ok(true)
    }
}
