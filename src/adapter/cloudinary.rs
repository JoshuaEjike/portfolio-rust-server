use crate::{error::api_error::ApiErrors, port::image_upload::ImageUploader};
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct CloudinaryUploader {
    client: Client,
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloudinaryUploadResponse {
    pub secure_url: String,
    pub public_id: String,
}

impl CloudinaryUploader {
    pub fn new(
        cloud_name: impl Into<String>,
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            cloud_name: cloud_name.into(),
            api_key: api_key.into(),
            api_secret: api_secret.into(),
        }
    }

    /// ✅ Cloudinary signature: plain SHA1 of "k=v&k2=v2...{api_secret}"
    fn sign_params(&self, params: &BTreeMap<&str, String>) -> String {
        let mut pieces = Vec::with_capacity(params.len());
        for (k, v) in params {
            pieces.push(format!("{k}={v}"));
        }

        let to_sign = format!("{}{}", pieces.join("&"), self.api_secret);
        let mut hasher = Sha1::new();
        hasher.update(to_sign.as_bytes());
        format!("{:x}", hasher.finalize()) // hex digest
    }
}

#[async_trait]
impl ImageUploader for CloudinaryUploader {
    async fn upload_bytes(
        &self,
        filename: &str,
        bytes: &[u8],
    ) -> Result<CloudinaryUploadResponse, ApiErrors> {
        // 1. Prepare timestamp
        let timestamp = Utc::now().timestamp().to_string();

        // 2. Prepare params for signature
        let mut params = BTreeMap::new();
        params.insert("timestamp", timestamp.clone());
        // optionally add: params.insert("folder", "my_uploads".to_string());

        // 3. Generate SHA1 signature
        let signature = self.sign_params(&params);

        // 4. Build Cloudinary URL
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            self.cloud_name
        );

        // 5. Build form data
        let form = reqwest::multipart::Form::new()
            .text("api_key", self.api_key.clone())
            .text("timestamp", timestamp)
            .text("signature", signature)
            .part(
                "file",
                reqwest::multipart::Part::bytes(bytes.to_vec()).file_name(filename.to_string()),
            );

        // 6. Send request
        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                ApiErrors::InternalServerError(format!("Cloudinary request failed: {e}"))
            })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let txt = resp.text().await.unwrap_or_default();
            return Err(ApiErrors::InternalServerError(format!(
                "Cloudinary error: status={status} body={txt}"
            )));
        }

        // 7. Parse response
        let body: CloudinaryUploadResponse = resp.json().await.map_err(|e| {
            ApiErrors::InternalServerError(format!("Cloudinary JSON parse error: {e}"))
        })?;

        Ok(body)
    }
}
