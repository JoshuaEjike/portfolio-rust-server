use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::error::AuthError;

use super::{
    email::Email, name::Name, password::Password, phone_number::PhoneNumber, roles::Roles,
    uuid_lib::UserId,
};

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
    pub fn new(
        name: Name,
        email: Email,
        phone_number: Option<PhoneNumber>,
        roles: Roles,
        password: Password,
        created_by: Option<UserId>,
        created_by_name: Option<Name>,
        created_by_email: Option<Email>,
        // edited_by: Option<UserId>,
        // edited_by_name: Option<Name>,
        // edited_by_email: Option<Email>,
    ) -> Result<Self, AuthError> {
        let hash_password =
            hash(password.as_str(), DEFAULT_COST).map_err(|_| AuthError::HashError)?;

        let created_at = chrono::Utc::now().naive_utc();

        let details = Self {
            id: UserId(Uuid::new_v4()),
            name,
            email,
            phone_number,
            roles,
            password: Some(hash_password),
            created_by,
            created_by_email,
            created_by_name,
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
