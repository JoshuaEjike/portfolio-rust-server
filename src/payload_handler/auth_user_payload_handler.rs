use crate::{
    error::api_error::ApiErrors,
    payload_description::user_payload_description::{ValidatedLogin, ValidatedRegister},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub roles: Option<String>,
}

impl RegisterRequest {
    pub fn validate(self) -> Result<ValidatedRegister, ApiErrors> {
        let email = self
            .email
            .ok_or_else(|| ApiErrors::BadRequest("Email is required".to_string()))?;

        let password = self
            .password
            .ok_or_else(|| ApiErrors::BadRequest("Password is required".to_string()))?;

        let name = self
            .name
            .ok_or_else(|| ApiErrors::BadRequest("Name is required".to_string()))?;

        let roles = self
            .roles
            .ok_or_else(|| ApiErrors::BadRequest("Roles is required".to_string()))?;

        Ok(ValidatedRegister {
            email,
            password,
            name,
            phone_number: self.phone_number,
            roles,
        })
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

impl LoginRequest {
    pub fn validate(self) -> Result<ValidatedLogin, ApiErrors> {
        let email = self
            .email
            .ok_or_else(|| ApiErrors::BadRequest("Email is required".to_string()))?;

        let password = self
            .password
            .ok_or_else(|| ApiErrors::BadRequest("Password is required".to_string()))?;

        Ok(ValidatedLogin { email, password })
    }
}
