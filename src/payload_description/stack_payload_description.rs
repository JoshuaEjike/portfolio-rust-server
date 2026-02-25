use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::stack::DirectStackDetails,
    fields::{Email, Text},
};

#[derive(Debug, Deserialize)]
pub struct UpdateStackPayload {
    pub title: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug)]
pub struct UpdateStack {
    pub id: Uuid,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct CreateStackData {
    pub title: Text,
    pub slug: Text,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

pub struct ValidatedCreateStackData {
    pub title: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingAllStack {
    pub message: String,
    pub stacks: Vec<DirectStackDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingSingleStack {
    pub message: String,
    pub stack: Option<DirectStackDetails>,
}
