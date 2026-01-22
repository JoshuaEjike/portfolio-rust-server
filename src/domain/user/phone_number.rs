use serde::Serialize;

use crate::error::AuthError;

#[derive(Debug, Serialize, Clone)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    pub fn new(value: &str) -> Result<Self, AuthError> {
        let value = value.trim();

        // Must start with '+'
        if !value.starts_with('+') {
            return Err(AuthError::InvalidPhoneNumberFormat);
        }

        // Remove '+' and check remaining characters
        let digits = &value[1..];

        if !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err(AuthError::InvalidPhoneNumberFormat);
        }

        // E.164 length rule: max 15 digits (min ~10 is practical)
        if digits.len() < 10 {
            return Err(AuthError::PhoneNumberTooShort);
        }

        if digits.len() > 15 {
            return Err(AuthError::PhoneNumberTooLong);
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
