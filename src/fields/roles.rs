use serde::Serialize;

use crate::error::api_error::ApiErrors;

#[derive(Debug, Serialize, Clone)]
pub enum Roles {
    Root,
    Mid,
    Normal,
}

impl Roles {
    pub fn new(value: &str) -> Result<Self, ApiErrors> {
        match value {
            "root" => Ok(Self::Root),
            "mid" => Ok(Self::Mid),
            "normal" => Ok(Self::Normal),
            _ => Err(ApiErrors::BadRequest(
                "admin roles accepted are root, mid, normal.".to_string(),
            )),
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
