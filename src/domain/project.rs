use crate::{
    error::api_error::ApiErrors,
    fields::{Email, Text},
    payload_description::project_payload_description::CreateProjectData,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct DirectProjectDetails {
    pub id: Uuid,
    pub title: Text,
    pub description: Text,
    pub stack: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub title: Text,
    pub description: Text,
    pub stack: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Project {
    pub fn new(data: CreateProjectData) -> Result<Self, ApiErrors> {
        let created_at = chrono::Utc::now().naive_utc();

        let id = Uuid::new_v4();

        let details = Self {
            id,
            title: data.title,
            description: data.description,
            content: data.content,
            stack: data.stack,
            image: data.image,
            image_id: data.image_id,
            created_by: data.created_by,
            created_by_email: data.created_by_email,
            created_by_name: data.created_by_name,
            created_at,
            updated_at: created_at,
        };

        Ok(details)
    }
}
