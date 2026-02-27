use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::project::{DirectProjectDetails, Project},
    error::api_error::ApiErrors,
    payload_description::project_payload_description::{CreateProjectData, UpdateProject},
    port::project_db::ProjectDBServices,
};

pub struct ProjectServices {
    repo: Arc<dyn ProjectDBServices + Send + Sync>,
}

impl ProjectServices {
    pub fn new(repo: Arc<dyn ProjectDBServices + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_project(&self, data: CreateProjectData) -> Result<String, ApiErrors> {
        if self
            .repo
            .find_by_title(data.title.as_str())
            .await?
            .is_some()
        {
            return Err(ApiErrors::NotFound("project does not exist".to_string()));
        }

        let project = Project::new(data)?;

        let id = self.repo.create_project(&project).await?;

        Ok(id.into())
    }

    pub async fn get_all_project(&self) -> Result<Vec<DirectProjectDetails>, ApiErrors> {
        let project_data = self.repo.find_all_project().await?;

        Ok(project_data)
    }

    pub async fn find_single_project(
        &self,
        project_id: &Uuid,
    ) -> Result<Option<DirectProjectDetails>, ApiErrors> {
        let project = self
            .repo
            .find_by_id(project_id)
            .await?
            .ok_or(ApiErrors::NotFound("Project not found".to_string()))?;

        Ok(Some(DirectProjectDetails {
            id: project.id,
            title: project.title,
            description: project.description,
            stack: project.stack,
            content: project.content,
            image: project.image,
            image_id: project.image_id,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }))
    }

    pub async fn delete_project(&self, project_id: &Uuid) -> Result<bool, ApiErrors> {
        let project_data = self.repo.delete_project(project_id).await?;

        if !project_data {
            return Err(ApiErrors::NotFound("Project not found".to_string()));
        }

        Ok(true)
    }

    pub async fn update_project(&self, project: UpdateProject) -> Result<bool, ApiErrors> {
        if self.repo.find_by_id(&project.project_id).await?.is_none() {
            return Err(ApiErrors::NotFound("Stack not found".to_string()));
        }

        let project_data = self.repo.update_project(project).await?;

        if !project_data {
            return Err(ApiErrors::NotFound("Project not found".to_string()));
        }

        Ok(true)
    }
}
