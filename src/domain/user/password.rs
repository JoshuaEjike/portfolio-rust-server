use crate::error::AuthError;

#[derive(Debug)]
pub struct Password(pub String);

impl Password {
    pub fn new(value: &str) -> Result<Self, AuthError> {
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

        if value.len() < 8 {
            return Err(AuthError::TooShort);
        } else if !value.chars().any(|c| c.is_uppercase()) {
            return Err(AuthError::MissingUppercase);
        } else if !value.chars().any(|c| c.is_numeric()) {
            return Err(AuthError::MissingNumber);
        } else if !value.chars().any(|c| special_chars.contains(c)) {
            return Err(AuthError::MissingSpecialChar);
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
