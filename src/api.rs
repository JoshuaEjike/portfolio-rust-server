use axum::Router;

use crate::{api::user_api_routers::user_api_router, state::AppState};

pub mod user_api_routers;

pub fn app_apis(state: AppState) -> Router {
    Router::new().nest(
        "/api/v1",
        Router::new().nest("/auth", user_api_router(state.clone())),
    )
}
