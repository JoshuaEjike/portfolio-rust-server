use crate::{
    error::api_error::ApiErrors,
    payload_description::gobal_response_description::ResponseForImageUpload, state::AppState,
};

use axum::{
    Json,
    extract::{Multipart, State},
};
use base64::Engine as _;
use base64::engine::general_purpose;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base64UploadRequest {
    pub file_name: String,
    pub file_data: String,
}

pub async fn formdata_image_upload_router_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let field = multipart
        .next_field()
        .await
        .map_err(|_| ApiErrors::InternalServerError("Invalid form".to_string()))?
        .ok_or_else(|| ApiErrors::BadRequest("No file provided".to_string()))?;

    let filename = field
        .file_name()
        .map(|t| t.to_string())
        .ok_or_else(|| ApiErrors::BadRequest("No file provided".to_string()))?;

    let bytes = field
        .bytes()
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to read file".to_string()))?;

    let result = state
        .image_service
        .upload_through_file(filename, bytes.to_vec())
        .await?;

    let response = ResponseForImageUpload {
        message: "success".to_string(),
        data: result,
    };

    Ok(Json(serde_json::json!(response)))
}

pub async fn base64_image_upload_router_handler(
    State(state): State<AppState>,
    Json(payload): Json<Base64UploadRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let clean_data = if let Some(idx) = payload.file_data.find(",") {
        &payload.file_data[idx + 1..]
    } else {
        &payload.file_data
    };

    // Decode base64 → bytes
    let bytes = general_purpose::STANDARD
        .decode(clean_data)
        .map_err(|_| ApiErrors::BadRequest("Invalid base64 data".to_string()))?;

    // Upload to Cloudinary via service
    let result = state
        .image_service
        .upload_through_file(payload.file_name, bytes)
        .await?;

    let response = ResponseForImageUpload {
        message: "success".to_string(),
        data: result,
    };

    Ok(Json(serde_json::json!(response)))
}
