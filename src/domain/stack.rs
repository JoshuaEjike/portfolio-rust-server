use crate::{
    error::api_error::ApiErrors,
    fields::{Email, Text},
    payload_description::CreateStackData,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct DirectStackDetails {
    pub id: Uuid,
    pub title: Text,
    pub slug: Text,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub id: Uuid,
    pub title: Text,
    pub slug: Text,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Stack {
    pub fn new(data: CreateStackData) -> Result<Self, ApiErrors> {
        let created_at = chrono::Utc::now().naive_utc();

        let id = Uuid::new_v4();

        let details = Self {
            id,
            title: data.title,
            slug: data.slug,
            created_by: data.created_by,
            created_by_email: data.created_by_email,
            created_by_name: data.created_by_name,
            created_at,
            updated_at: created_at,
        };

        Ok(details)
    }
}
