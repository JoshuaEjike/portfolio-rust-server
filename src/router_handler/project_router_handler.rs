use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::api_error::ApiErrors,
    extractor::{
        auth_extractor::AuthUser,
        project_extractor::{ProjectCreateInput, ProjectUpateInput},
    },
    fields::Text,
    payload_description::{
        RequestUserIdPayload, SuccessMessageResponse,
        project_payload_description::{
            CreateProjectData, ResponseForGettingAllProject, ResponseForGettingSingleProject,
            UpdateProject,
        },
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn create_project_router(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    payload: ProjectCreateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let title = Text::new(&payload.title)?;

    let description = Text::new(&payload.description)?;

    let stack = Text::new(&payload.stack)?;

    let project = CreateProjectData {
        title,
        description,
        stack,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    let result = state.project_services.create_project(project).await?;

    let success_response = SuccessMessageResponse {
        message: format!("Stack created: {result}"),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_all_project_router(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let projects = state.project_services.get_all_project().await?;

    let success_response = ResponseForGettingAllProject {
        message: "success".to_string(),
        projects,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_single_project_router(
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let project = state.project_services.find_single_project(&id).await?;

    let success_response = ResponseForGettingSingleProject {
        message: "success".to_string(),
        project,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_delete_project_router(
    _: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    state.project_services.delete_project(&id).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_update_project_router(
    AuthUser {
        id, email, name, ..
    }: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
    payload: ProjectUpateInput,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let project_id =
        Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    let updated_project = UpdateProject {
        project_id,
        description: payload.description,
        stack: payload.stack,
        content: payload.content,
        image: payload.image,
        image_id: payload.image_id,
        edited_by: id,
        edited_by_name: name.as_str().to_string(),
        edited_by_email: email.as_str().to_string(),
    };

    state
        .project_services
        .update_project(updated_project)
        .await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}
