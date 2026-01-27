use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{error::AuthError, payload_description::SignUpUserData};

use super::{
    email::Email, name::Name, password::Password, phone_number::PhoneNumber, roles::Roles,
    uuid_lib::UserId,
};

#[derive(Debug, Serialize, Clone)]
pub struct DirectUsersDetails {
    pub id: UserId,
    pub name: Name,
    pub email: Email,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Roles,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct Users {
    pub id: UserId,
    pub name: Name,
    pub email: Email,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<String>,
    pub roles: Roles,
    pub created_by: Option<UserId>,
    pub created_by_name: Option<Name>,
    pub created_by_email: Option<Email>,
    pub edited_by: Option<UserId>,
    pub edited_by_name: Option<Name>,
    pub edited_by_email: Option<Email>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Users {
    pub fn new(data: SignUpUserData) -> Result<Self, AuthError> {
        let hash_password =
            hash(data.password.as_str(), DEFAULT_COST).map_err(|_| AuthError::HashError)?;

        let created_at = chrono::Utc::now().naive_utc();

        let details = Self {
            id: UserId(Uuid::new_v4()),
            name: data.name,
            email: data.email,
            phone_number: data.phone_number,
            roles: data.roles,
            password: Some(hash_password),
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

    pub fn verify_password(&self, password: &Password) -> bool {
        match &self.password {
            Some(hash) => verify(password.as_str(), hash).unwrap_or(false),
            None => false,
        }
    }
}
