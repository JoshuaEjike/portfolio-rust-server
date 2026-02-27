use crate::{
    error::api_error::ApiErrors,
    fields::{Email, Text},
    payload_description::blog_payload_description::CreateBlogData,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct DirectBlogDetails {
    pub id: Uuid,
    pub title: Text,
    pub description: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Blog {
    pub id: Uuid,
    pub title: Text,
    pub description: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Blog {
    pub fn new(data: CreateBlogData) -> Result<Self, ApiErrors> {
        let created_at = chrono::Utc::now().naive_utc();

        let id = Uuid::new_v4();

        let details = Self {
            id,
            title: data.title,
            description: data.description,
            content: data.content,
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
