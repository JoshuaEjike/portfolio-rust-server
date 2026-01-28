use crate::{
    domain::{
        user::{Email, Name},
        uuid_lib::Id,
    },
    error::stack_error::StackError,
    payload_description::CreateStackData,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct DirectStackDetails {
    pub id: Id,
    pub title: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub id: Id,
    pub title: String,
    pub slug: String,
    pub created_by: Option<Id>,
    pub created_by_name: Option<Name>,
    pub created_by_email: Option<Email>,
    pub edited_by: Option<Id>,
    pub edited_by_name: Option<Name>,
    pub edited_by_email: Option<Email>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Stack {
    pub fn new(data: CreateStackData) -> Result<Self, StackError> {
        let created_at = chrono::Utc::now().naive_utc();

        let details = Self {
            id: Id(Uuid::new_v4()),
            title: data.title,
            slug: data.slug,
            created_by: data.created_by,
            created_by_email: data.created_by_email,
            created_by_name: data.created_by_name,
            edited_by: None,
            edited_by_email: None,
            edited_by_name: None,
            created_at,
            updated_at: created_at,
        };

        Ok(details)
    }
}
