use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::blog::{Blog, DirectBlogDetails},
    error::api_error::ApiErrors,
    payload_description::blog_payload_description::{CreateBlogData, UpdateBlog},
    port::blog_db::BlogDBServices,
};

pub struct BlogServices {
    repo: Arc<dyn BlogDBServices + Send + Sync>,
}

impl BlogServices {
    pub fn new(repo: Arc<dyn BlogDBServices + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_blog(&self, data: CreateBlogData) -> Result<String, ApiErrors> {
        if self
            .repo
            .find_by_title(data.title.as_str())
            .await?
            .is_some()
        {
            return Err(ApiErrors::NotFound("blog does not exist".to_string()));
        }

        let blog = Blog::new(data)?;

        let id = self.repo.create_blog(&blog).await?;

        Ok(id.into())
    }

    pub async fn get_all_blog(&self) -> Result<Vec<DirectBlogDetails>, ApiErrors> {
        let blog_data = self.repo.find_all_blog().await?;

        Ok(blog_data)
    }

    pub async fn find_single_blog(
        &self,
        blog_id: &Uuid,
    ) -> Result<Option<DirectBlogDetails>, ApiErrors> {
        let blog = self
            .repo
            .find_by_id(blog_id)
            .await?
            .ok_or(ApiErrors::NotFound("Blog not found".to_string()))?;

        Ok(Some(DirectBlogDetails {
            id: blog.id,
            title: blog.title,
            description: blog.description,
            content: blog.content,
            image: blog.image,
            image_id: blog.image_id,
            created_at: blog.created_at,
            updated_at: blog.updated_at,
        }))
    }

    pub async fn delete_blog(&self, blog_id: &Uuid) -> Result<bool, ApiErrors> {
        let blog_data = self.repo.delete_blog(blog_id).await?;

        if !blog_data {
            return Err(ApiErrors::NotFound("Blog not found".to_string()));
        }

        Ok(true)
    }

    pub async fn update_blog(&self, blog: UpdateBlog) -> Result<bool, ApiErrors> {
        if self.repo.find_by_id(&blog.blog_id).await?.is_none() {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        let user_data = self.repo.update_blog(blog).await?;

        if !user_data {
            return Err(ApiErrors::NotFound("Blog not found".to_string()));
        }

        Ok(true)
    }
}
