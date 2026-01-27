pub mod user_api_routers;

use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tower::ServiceBuilder;

use crate::{
    api::user_api_routers::user_api_router, error::handle_404_with_path,
    payload_description::ErrorResponse, state::AppState,
};

pub fn app_apis(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new().nest("/auth", user_api_router(state.clone())),
        )
        .fallback(handle_404_with_path)
        .layer(
            ServiceBuilder::new()
                .map_response(|res: Response| {
                    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
                        // Replace the default 405 response
                        let body = ErrorResponse {
                            message: "Method not allowed for this route".to_string(),
                        };
                        (StatusCode::METHOD_NOT_ALLOWED, Json(body)).into_response()
                    } else {
                        res
                    }
                })
                .into_inner(),
        )
}
