use serde::Deserialize;

use crate::{error::api_error::ApiErrors, payload_description::ValidatedCreateStackData};

#[derive(Deserialize)]
pub struct StackCreateRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
}

impl StackCreateRequest {
    pub fn validate(self) -> Result<ValidatedCreateStackData, ApiErrors> {
        let title = self
            .title
            .ok_or_else(|| ApiErrors::BadRequest("Title is required".to_string()))?;

        let slug = self
            .slug
            .ok_or_else(|| ApiErrors::BadRequest("Slug is required".to_string()))?;

        Ok(ValidatedCreateStackData { title, slug })
    }
}
