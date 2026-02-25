use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::fields::{Email, Password, PhoneNumber, Roles, Text};

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub roles: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUser {
    pub id: Uuid,
    pub name: Option<Text>,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<Password>,
    pub roles: Option<Roles>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct UpdateUserDetails {
    pub id: Uuid,
    pub name: Option<Text>,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<String>,
    pub roles: Option<Roles>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct SignUpUserData {
    pub name: Text,
    pub email: Email,
    pub password: Password,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Roles,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidatedRegister {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub roles: String,
}

#[derive(Debug, Deserialize)]
pub struct ValidatedLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestUserEmailPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestUserIdPayload {
    pub id: String,
}
