use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    router_handler::project_router_handler::{
        create_project_router, get_all_project_router, get_delete_project_router,
        get_single_project_router, get_update_project_router,
    },
    state::AppState,
};

pub fn project_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_project_router))
        .route("/all", get(get_all_project_router))
        .route(
            "/detail/{id}",
            get(get_single_project_router)
                .patch(get_update_project_router)
                .delete(get_delete_project_router),
        )
        .with_state(state)
}
