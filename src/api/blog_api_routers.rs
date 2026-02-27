use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    router_handler::blog_router_handler::{
        create_blog_router, get_all_blog_router, get_delete_blog_router, get_single_blog_router,
        get_update_blog_router,
    },
    state::AppState,
};

pub fn blog_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_blog_router))
        .route("/all", get(get_all_blog_router))
        .route(
            "/detail/{id}",
            get(get_single_blog_router)
                .patch(get_update_blog_router)
                .delete(get_delete_blog_router),
        )
        .with_state(state)
}
