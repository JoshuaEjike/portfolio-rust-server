use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::stack::{DirectStackDetails, Stack},
    error::api_error::ApiErrors,
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

    pub async fn create_stacks(&self, data: CreateStackData) -> Result<String, ApiErrors> {
        if self
            .repo
            .find_by_title(data.title.as_str())
            .await?
            .is_some()
        {
            return Err(ApiErrors::NotFound("stack does not exist".to_string()));
        }

        let stack = Stack::new(data)?;

        let id = self.repo.create_stack(&stack).await?;

        Ok(id.into())
    }

    // this is to get all stack
    pub async fn get_all_stack(&self) -> Result<Vec<DirectStackDetails>, ApiErrors> {
        let stack_data = self.repo.find_all_stack().await?;

        Ok(stack_data)
    }

    pub async fn find_single_stack(
        &self,
        stack_id: &Uuid,
    ) -> Result<Option<DirectStackDetails>, ApiErrors> {
        let user = self
            .repo
            .find_by_id(stack_id)
            .await?
            .ok_or(ApiErrors::NotFound("Stack not found".to_string()))?;

        Ok(Some(DirectStackDetails {
            id: user.id,
            title: user.title,
            slug: user.slug,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }))
    }

    pub async fn delete_stack(&self, stack_id: &Uuid) -> Result<bool, ApiErrors> {
        let stack_data = self.repo.delete_stack(stack_id).await?;

        if !stack_data {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        Ok(true)
    }

    pub async fn update_stack(&self, stack: UpdateStack) -> Result<bool, ApiErrors> {
        if self.repo.find_by_id(&stack.id).await?.is_none() {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        let user_data = self.repo.update_stack(stack).await?;

        if !user_data {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        Ok(true)
    }
}
