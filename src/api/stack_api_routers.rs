use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    router_handler::stack_router_handler::{
        create_stack_router, get_all_stack_router, get_delete_stack_router,
        get_single_stack_router, get_update_stack_router,
    },
    state::AppState,
};

pub fn stack_api_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_stack_router))
        .route("/get_all_stack", get(get_all_stack_router))
        .route(
            "/single_stack/{id}",
            get(get_single_stack_router)
                .delete(get_delete_stack_router)
                .patch(get_update_stack_router),
        )
        .with_state(state)
}
