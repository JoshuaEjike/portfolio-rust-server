use serde::{Deserialize, Serialize};

use crate::domain::user::{Email, Name, Password, PhoneNumber, UserId};

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
    pub user_id: UserId,
    pub name: Option<Name>,
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<Password>,
    pub roles: Option<String>,
    pub edited_by: Option<UserId>,
    pub edited_by_name: Option<Name>,
    pub edited_by_email: Option<Email>,
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
