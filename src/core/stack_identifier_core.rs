use std::sync::Arc;

use crate::{application::StackServices, error::api_error::ApiErrors};

pub async fn ensure_stack_exists(
    stack_title: String,
    stack_services: Arc<StackServices>,
) -> Result<(), ApiErrors> {
    stack_services
        .find_single_stack_by_title(stack_title)
        .await?;

    Ok(())
}
