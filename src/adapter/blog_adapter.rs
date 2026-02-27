use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::blog::{Blog, DirectBlogDetails},
    error::api_error::ApiErrors,
    fields::Text,
    payload_description::blog_payload_description::UpdateBlog,
    port::blog_db::BlogDBServices,
};

pub struct PostgreBlogRepository {
    pool: PgPool,
}

impl PostgreBlogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BlogDBServices for PostgreBlogRepository {
    async fn create_blog(&self, blog: &Blog) -> Result<Uuid, ApiErrors> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO blog (
                    id, title, description,
                    content, image, image_id,
                    created_by, created_by_name, created_by_email,
                    created_at, updated_at
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            "#,
            blog.id,
            blog.title.as_str(),
            blog.description.as_str(),
            blog.content,
            blog.image,
            blog.image_id,
            blog.created_by,
            blog.created_by_name.as_str(),
            blog.created_by_email.as_str(),
            blog.created_at,
            blog.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(blog.id)
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<DirectBlogDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, title, description, content, image, image_id, created_at, updated_at FROM blog WHERE title=$1"#,
            title
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::Unauthorized("Invalid credentials".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectBlogDetails {
                    id: d.id,
                    title: Text(d.title),
                    description: Text(d.description),
                    content: d.content,
                    image: d.image,
                    image_id: d.image_id,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn find_by_id(&self, blog_id: &Uuid) -> Result<Option<DirectBlogDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, title, description, content, image, image_id, created_at, updated_at FROM blog WHERE id = $1"#,
            blog_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("User not found".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectBlogDetails {
                    id: d.id,
                    title: Text(d.title),
                    description: Text(d.description),
                    content: d.content,
                    image: d.image,
                    image_id: d.image_id,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn delete_blog(&self, blog_id: &Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM blog where id = $1", blog_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Delete failed".to_string()))?;

        let deleted = result.rows_affected() > 0;

        Ok(deleted)
    }

    async fn update_blog(&self, blog: UpdateBlog) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(
            r#"
        UPDATE blog
        SET description = COALESCE($1, description),
            content = COALESCE($2, content),
            image = COALESCE($3, image),
            image_id = COALESCE($4, image_id),
            edited_by = $5,
            edited_by_name = $6,
            edited_by_email = $7,
            updated_at = NOW()
        WHERE id = $8
        "#,
            blog.description.as_ref().map(|s| s.as_str()),
            blog.content,
            blog.image,
            blog.image_id,
            blog.edited_by,
            blog.edited_by_name,
            blog.edited_by_email,
            blog.blog_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_all_blog(&self) -> Result<Vec<DirectBlogDetails>, ApiErrors> {
        let rows = sqlx::query!(r#"SELECT id, title, description, content, image, image_id, created_at, updated_at FROM blog ORDER BY created_at DESC"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        rows.into_iter()
            .map(|blg| {
                Ok(DirectBlogDetails {
                    id: blg.id,
                    title: Text(blg.title),
                    description: Text(blg.description),
                    content: blg.content,
                    image: blg.image,
                    image_id: blg.image_id,
                    created_at: blg.created_at,
                    updated_at: blg.updated_at,
                })
            })
            .collect()
    }
}
