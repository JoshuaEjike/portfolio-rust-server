use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::project::DirectProjectDetails,
    fields::{Email, Text},
};

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub description: Option<String>,
    pub stack: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_name: Option<String>,
}

#[derive(Debug)]
pub struct UpdateProject {
    pub project_id: Uuid,
    pub description: Option<Text>,
    pub stack: Option<Text>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<String>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct CreateProjectData {
    pub title: Text,
    pub description: Text,
    pub stack: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

pub struct ValidatedCreateProjectData {
    pub title: String,
    pub description: String,
    pub stack: String,
    pub content: String,
    pub image: String,
    pub image_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingAllProject {
    pub message: String,
    pub projects: Vec<DirectProjectDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingSingleProject {
    pub message: String,
    pub project: Option<DirectProjectDetails>,
}
