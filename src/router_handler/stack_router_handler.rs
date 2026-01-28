use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{
    domain::uuid_lib::Id,
    error::error_manager,
    extract_or_early_return,
    payload_description::{
        CreateStackData, ErrorResponse, RequestUserIdPayload, ResponseForGettingAllStack,
        ResponseForGettingSingleStack, StackPayloadLoader, SuccessMessageResponse, UpdateStack,
        UpdateStackPayload,
    },
    state::AppState,
    utils::CurrentUser,
};

#[axum::debug_handler]
pub async fn create_stack_router(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload_data): Json<StackPayloadLoader>,
) -> impl IntoResponse {
    let current_user = match CurrentUser::from_headers(
        &headers,
        state.jwt_services.clone(),
        state.user_repo.clone(),
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            return error_manager(StatusCode::UNAUTHORIZED, err.to_string());
        }
    };

    let title = extract_or_early_return!(payload_data.title.ok_or("title is required"));

    let slug = extract_or_early_return!(payload_data.slug.ok_or("slug is required"));

    let stack = CreateStackData {
        title,
        slug,
        created_by: Some(current_user.0.id),
        created_by_name: Some(current_user.0.name),
        created_by_email: Some(current_user.0.email),
    };

    match state.stack_services.create_stacks(stack).await {
        Ok(_) => {
            let success_response = SuccessMessageResponse {
                message: "success".to_string(),
            };

            (StatusCode::CREATED, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

#[axum::debug_handler]
pub async fn get_all_stack_router(State(state): State<AppState>) -> impl IntoResponse {
    match state.stack_services.get_all_stack().await {
        Ok(stack) => {
            let success_response = ResponseForGettingAllStack {
                message: "success".to_string(),
                stack,
            };

            (StatusCode::OK, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

#[axum::debug_handler]
pub async fn get_single_stack_router(
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> impl IntoResponse {
    let id = match Id::from_str(&path.id) {
        Ok(data) => data,
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    };

    match state.stack_services.find_single_stack(&id).await {
        Ok(stack) => {
            let success_response = ResponseForGettingSingleStack {
                message: "success".to_string(),
                stack,
            };

            // ✅ Works fine in axum 0.8+
            (StatusCode::OK, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

#[axum::debug_handler]
pub async fn get_delete_stack_router(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<RequestUserIdPayload>,
) -> impl IntoResponse {
    let _ = match CurrentUser::from_headers(
        &headers,
        state.jwt_services.clone(),
        state.user_repo.clone(),
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            return error_manager(StatusCode::UNAUTHORIZED, err.to_string());
        }
    };

    let id = match Id::from_str(&path.id) {
        Ok(data) => data,
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    };

    match state.stack_services.delete_stack(&id).await {
        Ok(_stack_data) => {
            let success_response = SuccessMessageResponse {
                message: "success".to_string(),
            };

            // ✅ Works fine in axum 0.8+
            (StatusCode::OK, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

#[axum::debug_handler]
pub async fn get_update_stack_router(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<RequestUserIdPayload>,
    Json(payload_data): Json<UpdateStackPayload>,
) -> impl IntoResponse {
    let user = match CurrentUser::from_headers(
        &headers,
        state.jwt_services.clone(),
        state.user_repo.clone(),
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            return error_manager(StatusCode::UNAUTHORIZED, err.to_string());
        }
    };

    let user_id = match Id::from_str(&path.id) {
        Ok(data) => data,
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    };

    let updated_stack = UpdateStack {
        id: user_id.clone(),
        title: payload_data.title,
        slug: payload_data.slug,
        edited_by: user.0.id.as_uuid(),
        edited_by_name: user.0.email.as_str().to_string(),
        edited_by_email: user.0.name.as_str().to_string(),
    };

    match state.stack_services.update_stack(updated_stack).await {
        Ok(_users_data) => {
            let success_response = SuccessMessageResponse {
                message: "success".to_string(),
            };

            // ✅ Works fine in axum 0.8+
            (StatusCode::OK, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}
