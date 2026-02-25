use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    core::password_core::hashing_password,
    error::api_error::ApiErrors,
    fields::{Email, PhoneNumber, Roles, Text},
    payload_description::SignUpUserData,
};

#[derive(Debug, Serialize, Clone)]
pub struct DirectUsersDetails {
    pub id: Uuid,
    pub name: Text,
    pub email: Email,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Roles,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct DirectUsersAccess {
    pub id: Uuid,
    pub name: Text,
    pub email: Email,
    pub phone_number: Option<PhoneNumber>,
    pub password: String,
    pub roles: Roles,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Users {
    pub id: Uuid,
    pub name: Text,
    pub email: Email,
    pub phone_number: Option<PhoneNumber>,
    pub password: String,
    pub roles: Roles,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Users {
    pub fn new(data: SignUpUserData) -> Result<Self, ApiErrors> {
        let hash_password = hashing_password(data.password.as_str().to_string())?;

        let created_at = chrono::Utc::now().naive_utc();

        let id = Uuid::new_v4();

        let details = Self {
            id,
            name: data.name,
            email: data.email,
            phone_number: data.phone_number,
            roles: data.roles,
            password: hash_password,
            created_by: data.created_by,
            created_by_email: data.created_by_email,
            created_by_name: data.created_by_name,
            created_at,
            updated_at: created_at,
        };

        Ok(details)
    }
}
