use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::stack::{DirectStackDetails, Stack},
    error::api_error::ApiErrors,
    fields::Text,
    payload_description::UpdateStack,
    port::StackDBServices,
};

pub struct PostgreStackRepository {
    pool: PgPool,
}

impl PostgreStackRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StackDBServices for PostgreStackRepository {
    async fn create_stack(&self, stack: &Stack) -> Result<Uuid, ApiErrors> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO stack (
                    id, title, slug,
                    created_by, created_by_name, created_by_email,
                    created_at, updated_at
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
            "#,
            stack.id,
            stack.title.as_str(),
            stack.slug.as_str(),
            stack.created_by,
            stack.created_by_name.as_str(),
            stack.created_by_email.as_str(),
            stack.created_at,
            stack.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(stack.id)
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<DirectStackDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, title, slug, created_at, updated_at FROM stack WHERE title=$1"#,
            title
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::Unauthorized("Invalid credentials".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectStackDetails {
                    id: d.id,
                    title: Text(d.title),
                    slug: Text(d.slug),
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn find_by_id(&self, stack_id: &Uuid) -> Result<Option<DirectStackDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, title, slug, created_at, updated_at FROM stack WHERE id=$1"#,
            stack_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("User not found".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectStackDetails {
                    id: d.id,
                    title: Text(d.title),
                    slug: Text(d.slug),
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn delete_stack(&self, stack_id: &Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM stack where id = $1", stack_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Delete failed".to_string()))?;

        let deleted = result.rows_affected() > 0;

        Ok(deleted)
    }

    async fn update_stack(&self, stack: UpdateStack) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(
            r#"
        UPDATE stack
        SET title = COALESCE($1, title),
            slug = COALESCE($2, slug),
            edited_by = $3,
            edited_by_name = $4,
            edited_by_email = $5,
            updated_at = NOW()
        WHERE id = $6
        "#,
            stack.title.as_ref().map(|t| t),
            stack.slug.as_ref().map(|s| s),
            stack.edited_by,
            stack.edited_by_name,
            stack.edited_by_email,
            stack.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_all_stack(&self) -> Result<Vec<DirectStackDetails>, ApiErrors> {
        let rows = sqlx::query!(r#"SELECT id, title, slug, created_at, updated_at FROM stack"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        rows.into_iter()
            .map(|stack| {
                Ok(DirectStackDetails {
                    id: stack.id,
                    title: Text(stack.title),
                    slug: Text(stack.slug),
                    created_at: stack.created_at,
                    updated_at: stack.updated_at,
                })
            })
            .collect()
    }
}
