use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::{
        stack::{DirectStackDetails, Stack},
        uuid_lib::Id,
    },
    error::stack_error::StackError,
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
    async fn create_stack(&self, stack: &Stack) -> Result<(), StackError> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO stack (
                    id, title, slug,
                    created_by, created_by_name, created_by_email,
                    edited_by, edited_by_name, edited_by_email,
                    created_at, updated_at
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            "#,
            stack.id.as_uuid(),
            stack.title,
            stack.slug,
            stack.created_by.as_ref().map(|c| c.as_uuid()),
            stack.created_by_name.as_ref().map(|n| n.as_str()),
            stack.created_by_email.as_ref().map(|e| e.as_str()),
            stack.edited_by.as_ref().map(|c| c.as_uuid()),
            stack.edited_by_name.as_ref().map(|n| n.as_str()),
            stack.edited_by_email.as_ref().map(|e| e.as_str()),
            stack.created_at,
            stack.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| StackError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<Stack>, StackError> {
        let row = sqlx::query!(
            r#"SELECT id, title, slug, created_at, updated_at FROM stack WHERE title=$1"#,
            title
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StackError::DatabaseError(e.to_string()))?;

        Ok(row
            .map(|d| {
                Ok(Stack {
                    id: Id(d.id),
                    title: d.title,
                    slug: d.slug,
                    created_by: None,
                    created_by_name: None,
                    created_by_email: None,
                    edited_by: None,
                    edited_by_name: None,
                    edited_by_email: None,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn find_by_id(&self, stack_id: &Id) -> Result<Option<DirectStackDetails>, StackError> {
        let row = sqlx::query!(
            r#"SELECT id, title, slug, created_at, updated_at FROM stack WHERE id=$1"#,
            stack_id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StackError::DatabaseError(e.to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectStackDetails {
                    id: Id(d.id),
                    title: d.title,
                    slug: d.slug,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn delete_stack(&self, stack_id: &Id) -> Result<bool, StackError> {
        let result = sqlx::query!("DELETE FROM stack where id = $1", stack_id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|err| StackError::DatabaseError(err.to_string()))?;

        let deleted = result.rows_affected() > 0;

        Ok(deleted)
    }

    async fn update_stack(&self, stack: UpdateStack) -> Result<bool, StackError> {
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
            stack.id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| StackError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_all_stack(&self) -> Result<Vec<DirectStackDetails>, StackError> {
        let rows = sqlx::query!(r#"SELECT id, title, slug, created_at, updated_at FROM stack"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StackError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|stack| {
                Ok(DirectStackDetails {
                    id: Id(stack.id),
                    title: stack.title,
                    slug: stack.slug,
                    created_at: stack.created_at,
                    updated_at: stack.updated_at,
                })
            })
            .collect()
    }
}
