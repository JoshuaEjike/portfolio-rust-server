use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

use crate::error::api_error::ApiErrors;

#[derive(Debug, Serialize, Clone)]
pub struct Text(pub String);

impl Text {
    pub fn new(value: &str) -> Result<Self, ApiErrors> {
        if name_regex().is_match(value) {
            Ok(Self(value.to_string()))
        } else {
            Err(ApiErrors::TextValidation(
                "character must be alphabetic".to_string(),
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn name_regex() -> &'static Regex {
    static EMAIL_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[A-Za-z\s'-]+$").expect("Invalid email regex"));
    &EMAIL_RE
}
