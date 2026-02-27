use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::api_error::ApiErrors,
    extractor::{
        auth_extractor::AuthUser,
        blog_extractor::{BlogCreateInput, BlogUpateInput},
    },
    fields::Text,
    payload_description::{
        RequestUserIdPayload, SuccessMessageResponse,
        blog_payload_description::{
            CreateBlogData, ResponseForGettingAllBlog, ResponseForGettingSingleBlog, UpdateBlog,
        },
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn create_blog_router(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    payload: BlogCreateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let title = Text::new(&payload.title)?;

    let description = Text::new(&payload.description)?;

    let blog = CreateBlogData {
        title,
        description,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    let result = state.blog_services.create_blog(blog).await?;

    let success_response = SuccessMessageResponse {
        message: format!("Stack created: {result}"),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_all_blog_router(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let blogs = state.blog_services.get_all_blog().await?;

    let success_response = ResponseForGettingAllBlog {
        message: "success".to_string(),
        blogs,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_single_blog_router(
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let blog = state.blog_services.find_single_blog(&id).await?;

    let success_response = ResponseForGettingSingleBlog {
        message: "success".to_string(),
        blog,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_delete_blog_router(
    _: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    state.blog_services.delete_blog(&id).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_update_blog_router(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
    payload: BlogUpateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let blog_id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let updated_blog = UpdateBlog {
        blog_id,
        description: payload.description,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        edited_by: id,
        edited_by_name: name.as_str().to_string(),
        edited_by_email: email.as_str().to_string(),
    };

    state.blog_services.update_blog(updated_blog).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}
