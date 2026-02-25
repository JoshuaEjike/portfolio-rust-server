use axum::{Json, extract::OriginalUri, http::Method};

use crate::error::api_error::ApiErrors;

pub async fn handle_404_with_path(
    method: Method,
    uri: OriginalUri,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    Err(ApiErrors::NotFound(format!(
        "Route {} {} does not exist",
        method, uri.0
    )))
}
