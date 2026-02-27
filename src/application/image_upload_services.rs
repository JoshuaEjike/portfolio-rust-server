use crate::{
    adapter::cloudinary::CloudinaryUploadResponse, error::api_error::ApiErrors,
    port::image_upload::ImageUploader,
};
use std::sync::Arc;

pub struct ImageUploadService {
    repo: Arc<dyn ImageUploader + Send + Sync>,
}

impl ImageUploadService {
    pub fn new(repo: Arc<dyn ImageUploader + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn upload_through_file(
        &self,
        filename: String,
        bytes: Vec<u8>,
    ) -> Result<CloudinaryUploadResponse, ApiErrors> {
        self.repo.upload_bytes(&filename, &bytes).await
    }
}
