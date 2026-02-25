use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::api_error::ApiErrors,
    extractor::auth_extractor::AuthUser,
    fields::Text,
    payload_description::{
        CreateStackData, RequestUserIdPayload, ResponseForGettingAllStack,
        ResponseForGettingSingleStack, SuccessMessageResponse, UpdateStack, UpdateStackPayload,
    },
    payload_handler::stack_payload_handler::StackCreateRequest,
    state::AppState,
};

#[axum::debug_handler]
pub async fn create_stack_router(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<StackCreateRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let payload_data = payload.validate()?;

    let title = Text::new(&payload_data.title)?;

    let slug = Text::new(&payload_data.slug)?;

    let stack = CreateStackData {
        title,
        slug,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    let result = state.stack_services.create_stacks(stack).await?;

    let success_response = SuccessMessageResponse {
        message: format!("Stack created: {result}"),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_all_stack_router(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let stacks = state.stack_services.get_all_stack().await?;

    let success_response = ResponseForGettingAllStack {
        message: "success".to_string(),
        stacks,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_single_stack_router(
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let stack = state.stack_services.find_single_stack(&id).await?;

    let success_response = ResponseForGettingSingleStack {
        message: "success".to_string(),
        stack,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_delete_stack_router(
    _: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    state.stack_services.delete_stack(&id).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_update_stack_router(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
    Json(payload_data): Json<UpdateStackPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let user_id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let updated_stack = UpdateStack {
        id: user_id.clone(),
        title: payload_data.title,
        slug: payload_data.slug,
        edited_by: id,
        edited_by_name: email.as_str().to_string(),
        edited_by_email: name.as_str().to_string(),
    };

    state.stack_services.update_stack(updated_stack).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}
