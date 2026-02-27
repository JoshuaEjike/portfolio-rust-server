use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::blog::{Blog, DirectBlogDetails},
    error::api_error::ApiErrors,
    payload_description::blog_payload_description::UpdateBlog,
};

#[async_trait]
pub trait BlogDBServices: Send + Sync {
    async fn create_blog(&self, blog: &Blog) -> Result<Uuid, ApiErrors>;
    async fn find_by_title(&self, title: &str) -> Result<Option<DirectBlogDetails>, ApiErrors>;
    async fn find_by_id(&self, blog_id: &Uuid) -> Result<Option<DirectBlogDetails>, ApiErrors>;
    async fn delete_blog(&self, blog_id: &Uuid) -> Result<bool, ApiErrors>;
    async fn update_blog(&self, blog: UpdateBlog) -> Result<bool, ApiErrors>;
    async fn find_all_blog(&self) -> Result<Vec<DirectBlogDetails>, ApiErrors>;
}
