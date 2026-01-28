use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    stack::DirectStackDetails,
    user::{Email, Name},
    uuid_lib::Id,
};

#[derive(Debug, Deserialize)]
pub struct UpdateStackPayload {
    pub title: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug)]
pub struct UpdateStack {
    pub id: Id,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct CreateStackData {
    pub title: String,
    pub slug: String,
    pub created_by: Option<Id>,
    pub created_by_name: Option<Name>,
    pub created_by_email: Option<Email>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StackPayloadLoader {
    pub title: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingAllStack {
    pub message: String,
    pub stack: Vec<DirectStackDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingSingleStack {
    pub message: String,
    pub stack: Option<DirectStackDetails>,
}
