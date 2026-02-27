use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose;

use crate::{
    adapter::cloudinary::CloudinaryUploadResponse,
    application::image_upload_services::ImageUploadService, error::api_error::ApiErrors,
};

pub async fn base64_image_uploader_core(
    base64: String,
    file_name: String,
    image_service: Arc<ImageUploadService>,
) -> Result<CloudinaryUploadResponse, ApiErrors> {
    let clean_data = if let Some(idx) = base64.find(",") {
        &base64[idx + 1..]
    } else {
        &base64
    };

    // Decode base64 → bytes
    let bytes = general_purpose::STANDARD
        .decode(clean_data)
        .map_err(|_| ApiErrors::BadRequest("Invalid base64 data".to_string()))?;

    // Upload to Cloudinary via service
    let result = image_service.upload_through_file(file_name, bytes).await?;

    Ok(result)
}
