use serde::Deserialize;

use crate::{
    error::api_error::ApiErrors,
    payload_description::blog_payload_description::ValidatedCreateBlogData,
};

#[derive(Deserialize)]
pub struct BlogCreateRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_name: Option<String>,
}

impl BlogCreateRequest {
    pub fn validate(self) -> Result<ValidatedCreateBlogData, ApiErrors> {
        let title = self
            .title
            .ok_or_else(|| ApiErrors::BadRequest("Title is required".to_string()))?;

        let description = self
            .description
            .ok_or_else(|| ApiErrors::BadRequest("Description is required".to_string()))?;

        let content = self
            .content
            .ok_or_else(|| ApiErrors::BadRequest("Content is required".to_string()))?;

        let image = self
            .image
            .ok_or_else(|| ApiErrors::BadRequest("Image is required".to_string()))?;

        let image_name = self
            .image_name
            .ok_or_else(|| ApiErrors::BadRequest("Image is required".to_string()))?;

        Ok(ValidatedCreateBlogData {
            title,
            description,
            content,
            image,
            image_name,
        })
    }
}
