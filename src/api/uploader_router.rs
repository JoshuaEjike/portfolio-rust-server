use crate::{
    router_handler::{base64_image_upload_router_handler, formdata_image_upload_router_handler},
    state::AppState,
};
use axum::{Router, routing::post};

pub fn uploader_router(state: AppState) -> Router {
    Router::new()
        .route("/form_data", post(formdata_image_upload_router_handler))
        .route("/base64", post(base64_image_upload_router_handler))
        .with_state(state)
}
