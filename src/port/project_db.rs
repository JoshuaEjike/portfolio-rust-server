use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::project::{DirectProjectDetails, Project},
    error::api_error::ApiErrors,
    payload_description::project_payload_description::UpdateProject,
};

#[async_trait]
pub trait ProjectDBServices: Send + Sync {
    async fn create_project(&self, project: &Project) -> Result<Uuid, ApiErrors>;
    async fn find_by_title(&self, title: &str) -> Result<Option<DirectProjectDetails>, ApiErrors>;
    async fn find_by_id(
        &self,
        project_id: &Uuid,
    ) -> Result<Option<DirectProjectDetails>, ApiErrors>;
    async fn delete_project(&self, project_id: &Uuid) -> Result<bool, ApiErrors>;
    async fn update_project(&self, project: UpdateProject) -> Result<bool, ApiErrors>;
    async fn find_all_project(&self) -> Result<Vec<DirectProjectDetails>, ApiErrors>;
}
