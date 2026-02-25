pub mod stack_api_routers;
pub mod user_api_routers;

use axum::{
     Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tower::ServiceBuilder;

use crate::{
    api::{stack_api_routers::stack_api_router, user_api_routers::user_api_router},
    error::{api_error::ApiErrors, handle_404_with_path},
    state::AppState,
};

pub fn app_apis(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/auth", user_api_router(state.clone()))
                .nest("/stack", stack_api_router(state.clone())),
        )
        .fallback(handle_404_with_path)
        .layer(
            ServiceBuilder::new()
                .map_response(|res: Response| {
                    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
                        ApiErrors::MethodNotAllowed("Method not allowed for this route".to_string())
                            .into_response()
                    } else {
                        res
                    }
                })
                .into_inner(),
        )
}
