use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;

use crate::error::api_error::ApiErrors;

#[derive(Debug, Serialize, Clone)]
pub struct Email(pub String);

impl Email {
    pub fn new(value: &str) -> Result<Self, ApiErrors> {
        if email_regex().is_match(value) {
            Ok(Self(value.to_owned()))
        } else {
            Err(ApiErrors::EmailValidation(value.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn email_regex() -> &'static Regex {
    static EMAIL_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").expect("Invalid email regex"));
    &EMAIL_RE
}
