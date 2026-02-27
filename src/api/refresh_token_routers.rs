use axum::{Router, routing::post};
use tower_cookies::CookieManagerLayer;

use crate::{
    router_handler::refresh_token_router_handler::{logout, refresh},
    state::AppState,
};

pub fn refresh_token_routers(state: AppState) -> Router {
    Router::new()
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
