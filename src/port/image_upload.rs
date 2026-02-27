use async_trait::async_trait;

use crate::{adapter::cloudinary::CloudinaryUploadResponse, error::api_error::ApiErrors};

#[async_trait]
pub trait ImageUploader: Send + Sync {
    async fn upload_bytes(
        &self,
        filename: &str,
        bytes: &[u8],
    ) -> Result<CloudinaryUploadResponse, ApiErrors>;
}
