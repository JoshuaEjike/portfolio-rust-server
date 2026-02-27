use axum::{
    Json,
    extract::{Path, State},
};
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    core::login_token_core::login_token_core,
    error::api_error::ApiErrors,
    extractor::auth_extractor::AuthUser,
    fields::{Email, Password, PhoneNumber, Roles, Text},
    payload_description::{
        AuthSuccessResponse, RequestUserEmailPayload, RequestUserIdPayload,
        ResponseForGettingSingleUsersPayload, SignUpUserData, SuccessMessageResponse, UpdateUser,
        UpdateUserPayload, gobal_response_description::ResponseForGettingUsersPayload,
    },
    payload_handler::auth_user_payload_handler::{LoginRequest, RegisterRequest},
    state::AppState,
};

#[axum::debug_handler]
pub async fn auth_user_sign_up_router(
    AuthUser {
        id,
        email,
        name,
        roles,
    }: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let payload_data = payload.validate()?;

    if payload_data.roles == "root" {
        return Err(ApiErrors::BadRequest(
            "This Admin level cans't be create".to_string(),
        ));
    }

    if roles.as_str() == "normal" {
        return Err(ApiErrors::BadRequest(
            "Becuase of your ADMIN Level you can not create a user.".to_string(),
        ));
    }

    let email_data = Email::new(&payload_data.email)?;

    let password = Password::new(&payload_data.password)?;

    let name_data = Text::new(&payload_data.name)?;

    let roles_data = Roles::new(&payload_data.roles)?;

    let phone_number = payload_data
        .phone_number
        .as_deref()
        .map(PhoneNumber::new)
        .transpose()?;

    let user = SignUpUserData {
        name: name_data,
        email: email_data,
        password,
        phone_number,
        roles: roles_data,
        created_by: id,
        created_by_name: name,
        created_by_email: email,
    };

    let token = state.auth_user_service.sign_up_user(user).await?;

    let success_response = AuthSuccessResponse {
        token,
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

pub async fn auth_user_sign_in_router(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let payload_data = payload.validate()?;

    let email = Email::new(&payload_data.email)?;
    let password = Password::new(&payload_data.password)?;

    let user = state
        .auth_user_service
        .sign_in_user(&email, &password)
        .await?;

    let tokens = login_token_core(state.refresh_token_service, cookies, user).await?;

    let success_response = AuthSuccessResponse {
        token: tokens.access_token,
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_all_users_router(
    _: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let users = state.auth_user_service.find_all_users().await?;

    let success_response = ResponseForGettingUsersPayload {
        message: "success".to_string(),
        users,
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_delete_user_router(
    _: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let user_id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    state.auth_user_service.delete_user(&user_id).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_update_user_router(
    AuthUser {
        id,
        email,
        name,
        roles,
    }: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserIdPayload>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let user_id = Uuid::parse_str(&path.id).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;

    if let Some(data) = payload.roles.clone()
        && data == "root"
    {
        return Err(ApiErrors::BadRequest(
            "This Admin level cans't be create".to_string(),
        ));
    }

    if roles.as_str() == "normal" {
        return Err(ApiErrors::BadRequest(
            "Becuase of your ADMIN Level you can not create a user.".to_string(),
        ));
    }

    let name_data = payload.name.as_deref().map(Text::new).transpose()?;

    let phone_number = payload
        .phone_number
        .as_deref()
        .and_then(|n| PhoneNumber::new(n).ok());

    let password = payload
        .password
        .as_deref()
        .and_then(|n| Password::new(n).ok());

    let roles = payload.roles.as_deref().and_then(|n| Roles::new(n).ok());

    let updated_user = UpdateUser {
        id: user_id,
        name: name_data,
        phone_number,
        password,
        roles,
        edited_by: id,
        edited_by_name: email.as_str().to_string(),
        edited_by_email: name.as_str().to_string(),
    };

    state.auth_user_service.update_user(updated_user).await?;

    let success_response = SuccessMessageResponse {
        message: "success".to_string(),
    };

    Ok(Json(serde_json::json!(success_response)))
}

#[axum::debug_handler]
pub async fn get_single_user_router(
    _: AuthUser,
    State(state): State<AppState>,
    Path(path): Path<RequestUserEmailPayload>,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let email = Email::new(&path.email)?;

    let user = state.auth_user_service.find_single_user(&email).await?;

    let success_response = ResponseForGettingSingleUsersPayload {
        message: "success".to_string(),
        user,
    };

    Ok(Json(serde_json::json!(success_response)))
}
