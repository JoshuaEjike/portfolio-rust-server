use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::AuthError;

#[derive(Debug, Clone)]
pub struct Email(pub String);

impl Email {
    pub fn new(value: &str) -> Result<Self, AuthError> {
        if email_regex().is_match(value) {
            Ok(Self(value.to_owned()))
        } else {
            Err(AuthError::EmailValidationError(value.to_string()))
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
