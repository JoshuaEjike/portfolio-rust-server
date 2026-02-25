use crate::error::api_error::ApiErrors;

#[derive(Debug, Clone)]
pub struct Password(pub String);

impl Password {
    pub fn new(value: &str) -> Result<Self, ApiErrors> {
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

        if value.len() < 8 {
            return Err(ApiErrors::PasswordFail(
                "Password must be 8 character length or above".to_string(),
            ));
        } else if !value.chars().any(|c| c.is_uppercase()) {
            return Err(ApiErrors::PasswordFail(
                "Password must contain atleast one uppercase character".to_string(),
            ));
        } else if !value.chars().any(|c| c.is_numeric()) {
            return Err(ApiErrors::PasswordFail(
                "Password must contain atleast one Ddigit character".to_string(),
            ));
        } else if !value.chars().any(|c| special_chars.contains(c)) {
            return Err(ApiErrors::PasswordFail(
                "Password must contain atleast one special character".to_string(),
            ));
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
