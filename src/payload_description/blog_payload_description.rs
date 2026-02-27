use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::blog::DirectBlogDetails,
    fields::{Email, Text},
};

#[derive(Deserialize)]
pub struct UpdateBlogRequest {
    pub description: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_name: Option<String>,
}

#[derive(Debug)]
pub struct UpdateBlog {
    pub blog_id: Uuid,
    pub description: Option<Text>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub image_id: Option<String>,
    pub edited_by: Uuid,
    pub edited_by_name: String,
    pub edited_by_email: String,
}

#[derive(Debug)]
pub struct CreateBlogData {
    pub title: Text,
    pub description: Text,
    pub content: String,
    pub image: String,
    pub image_id: String,
    pub created_by: Uuid,
    pub created_by_name: Text,
    pub created_by_email: Email,
}

pub struct ValidatedCreateBlogData {
    pub title: String,
    pub description: String,
    pub content: String,
    pub image: String,
    pub image_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingAllBlog {
    pub message: String,
    pub blogs: Vec<DirectBlogDetails>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResponseForGettingSingleBlog {
    pub message: String,
    pub blog: Option<DirectBlogDetails>,
}
