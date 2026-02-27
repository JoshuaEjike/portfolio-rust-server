pub mod blog_api_routers;
pub mod project_api_routers;
pub mod refresh_token_routers;
pub mod stack_api_routers;
pub mod uploader_router;
pub mod user_api_routers;

use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use tower::ServiceBuilder;

use crate::{
    api::{
        blog_api_routers::blog_api_router, project_api_routers::project_api_router,
        refresh_token_routers::refresh_token_routers, stack_api_routers::stack_api_router,
        uploader_router::uploader_router, user_api_routers::user_api_router,
    },
    error::{api_error::ApiErrors, handle_404_with_path},
    state::AppState,
};

pub fn app_apis(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/auth", user_api_router(state.clone()))
                .nest("/stack", stack_api_router(state.clone()))
                .nest("/blog", blog_api_router(state.clone()))
                .nest("/project", project_api_router(state.clone()))
                .nest("/refresh", refresh_token_routers(state.clone()))
                .nest("/upload", uploader_router(state.clone())),
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
