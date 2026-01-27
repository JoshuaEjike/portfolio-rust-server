use serde::Serialize;

use crate::error::AuthError;

#[derive(Debug, Serialize, Clone)]
pub enum Roles {
    Root,
    Mid,
    Normal,
}

impl Roles {
    pub fn new(value: &str) -> Result<Self, AuthError> {
        match value {
            "root" => Ok(Self::Root),
            "mid" => Ok(Self::Mid),
            "normal" => Ok(Self::Normal),
            _ => Err(AuthError::AdminRolesError),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Root => "root",
            Self::Mid => "mid",
            Self::Normal => "normal",
        }
    }
}
