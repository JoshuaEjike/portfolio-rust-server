use serde::Serialize;

use crate::error::AuthError;

#[derive(Debug, Serialize, Clone)]
pub enum Roles {
    ROOT,
    MID,
    NORMAL,
}

impl Roles {
    pub fn new(value: &str) -> Result<Self, AuthError> {
        match value {
            "root" => Ok(Self::ROOT),
            "mid" => Ok(Self::MID),
            "normal" => Ok(Self::NORMAL),
            _ => Err(AuthError::AdminRolesError),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::ROOT => "root",
            Self::MID => "mid",
            Self::NORMAL => "normal",
        }
    }
}
