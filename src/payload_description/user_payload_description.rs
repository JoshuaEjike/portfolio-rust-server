use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::user::{Email, Name, Password, PhoneNumber, Roles, UserId};

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub roles: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUser {
    pub id: UserId,
    pub name: Option<Name>,
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<Password>,
    pub roles: Option<Roles>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct SignUpUserData {
    pub name: Name,
    pub email: Email,
    pub password: Password,
    pub phone_number: Option<PhoneNumber>,
    pub roles: Roles,
    pub created_by: Option<UserId>,
    pub created_by_name: Option<Name>,
    pub created_by_email: Option<Email>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersPayloadLoader {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub roles: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserSigninPayload {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RequestUserEmailPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestUserIdPayload {
    pub id: String,
}
