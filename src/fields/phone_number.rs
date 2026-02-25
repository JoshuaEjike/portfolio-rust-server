use serde::Serialize;

use crate::error::api_error::ApiErrors;

#[derive(Debug, Serialize, Clone)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    pub fn new(value: &str) -> Result<Self, ApiErrors> {
        let value = value.trim();

        // Must start with '+'
        if !value.starts_with('+') {
            return Err(ApiErrors::BadRequest("Invalid phone number format. Expected an international phone number in E.164 format (e.g., +2348012345678).".to_string()));
        }

        // Remove '+' and check remaining characters
        let digits = &value[1..];

        if !digits.chars().all(|c| c.is_ascii_digit()) {
            return Err(ApiErrors::BadRequest("Invalid phone number format. Expected an international phone number in E.164 format (e.g., +2348012345678).".to_string()));
        }

        // E.164 length rule: max 15 digits (min ~10 is practical)
        if digits.len() < 10 {
            return Err(ApiErrors::BadRequest(
                "Phone number is too short. Expected a valid international phone number."
                    .to_string(),
            ));
        }

        if digits.len() > 15 {
            return Err(ApiErrors::BadRequest(
                "Phone number exceeds the maximum allowed length for international numbers."
                    .to_string(),
            ));
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
