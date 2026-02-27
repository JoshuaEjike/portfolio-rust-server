use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::project::{DirectProjectDetails, Project},
    error::api_error::ApiErrors,
    fields::Text,
    payload_description::project_payload_description::UpdateProject,
    port::project_db::ProjectDBServices,
};

pub struct PostgreProjectRepository {
    pool: PgPool,
}

impl PostgreProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectDBServices for PostgreProjectRepository {
    async fn create_project(&self, project: &Project) -> Result<Uuid, ApiErrors> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO project (
                    id, title, description, stack,
                    content, image, image_id,
                    created_by, created_by_name, created_by_email,
                    created_at, updated_at
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
            "#,
            project.id,
            project.title.as_str(),
            project.description.as_str(),
            project.stack.as_str(),
            project.content,
            project.image,
            project.image_id,
            project.created_by,
            project.created_by_name.as_str(),
            project.created_by_email.as_str(),
            project.created_at,
            project.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(project.id)
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<DirectProjectDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, title, description, stack, content, image, image_id, created_at, updated_at FROM project WHERE title=$1"#,
            title
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::Unauthorized("Invalid credentials".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectProjectDetails {
                    id: d.id,
                    title: Text(d.title),
                    description: Text(d.description),
                    stack: Text(d.stack),
                    content: d.content,
                    image: d.image,
                    image_id: d.image_id,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn find_by_id(
        &self,
        project_id: &Uuid,
    ) -> Result<Option<DirectProjectDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, title, description, stack, content, image, image_id, created_at, updated_at FROM project WHERE id = $1"#,
            project_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("Project not found".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectProjectDetails {
                    id: d.id,
                    title: Text(d.title),
                    description: Text(d.description),
                    stack: Text(d.stack),
                    content: d.content,
                    image: d.image,
                    image_id: d.image_id,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn delete_project(&self, project_id: &Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM project where id = $1", project_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Delete failed".to_string()))?;

        let deleted = result.rows_affected() > 0;

        Ok(deleted)
    }

    async fn update_project(&self, project: UpdateProject) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(
            r#"
        UPDATE project
        SET description = COALESCE($1, description),
        stack = COALESCE($2, stack),
            content = COALESCE($3, content),
            image = COALESCE($4, image),
            image_id = COALESCE($5, image_id),
            edited_by = $6,
            edited_by_name = $7,
            edited_by_email = $8,
            updated_at = NOW()
        WHERE id = $9
        "#,
            project.description.as_ref().map(|s| s.as_str()),
            project.stack.as_ref().map(|s| s.as_str()),
            project.content,
            project.image,
            project.image_id,
            project.edited_by,
            project.edited_by_name,
            project.edited_by_email,
            project.project_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_all_project(&self) -> Result<Vec<DirectProjectDetails>, ApiErrors> {
        let rows = sqlx::query!(r#"SELECT id, title, description, stack, content, image, image_id, created_at, updated_at FROM project ORDER BY created_at DESC"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        rows.into_iter()
            .map(|blg| {
                Ok(DirectProjectDetails {
                    id: blg.id,
                    title: Text(blg.title),
                    description: Text(blg.description),
                    stack: Text(blg.stack),
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
