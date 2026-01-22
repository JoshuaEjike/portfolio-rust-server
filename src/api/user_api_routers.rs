use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    router_handler::{auth_user_sign_in_router, auth_user_sign_up_router, get_all_users_router},
    state::AppState,
};

pub fn user_api_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(auth_user_sign_up_router))
        .route("/login", post(auth_user_sign_in_router))
        .route("/get_all_users", get(get_all_users_router))
        .with_state(state)
}
