use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{
    domain::user::{Email, Name, Password, PhoneNumber, Roles},
    error::error_manager,
    extract_or_early_return,
    payload_description::{
        AuthSuccessResponse, ErrorResponse, UserSigninPayload, UsersPayloadLoader,
    },
    payload_handler::auth_user_json_payload_handler,
    state::AppState,
    utils::CurrentUser,
};

#[axum::debug_handler]
pub async fn auth_user_sign_up_router(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload_data): Json<UsersPayloadLoader>,
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

    let email_str = extract_or_early_return!(payload_data.email.ok_or("email is required"));

    let name_str = extract_or_early_return!(payload_data.name.ok_or("Name is required"));

    let phone_number_str =
        extract_or_early_return!(payload_data.phone_number.ok_or("Phone number is required"));

    let roles_str = extract_or_early_return!(payload_data.roles.ok_or("Roles is required"));

    let password_str =
        extract_or_early_return!(payload_data.password.ok_or("Password is required"));

    if roles_str.as_str() == "root" {
        return error_manager(
            StatusCode::FORBIDDEN,
            "This Admin level cans't be create".to_string(),
        );
    }

    if current_user.0.roles.as_str() == "normal" {
        return error_manager(
            StatusCode::FORBIDDEN,
            "Becuase of your ADMIN Level you can not create a user.".to_string(),
        );
    }

    let email = extract_or_early_return!(Email::new(&email_str));
    let name = extract_or_early_return!(Name::new(&name_str));
    let phone_number = extract_or_early_return!(PhoneNumber::new(&phone_number_str));
    let roles = extract_or_early_return!(Roles::new(&roles_str));
    let password = extract_or_early_return!(Password::new(&password_str));

    match state
        .auth_user_service
        .sign_up_user(
            name,
            email,
            Some(phone_number),
            roles,
            password,
            Some(current_user.0.id),
            Some(current_user.0.name),
            Some(current_user.0.email),
        )
        .await
    {
        Ok(token) => {
            let success_response = AuthSuccessResponse {
                token,
                message: "success".to_string(),
            };

            (StatusCode::CREATED, Json(success_response)).into_response()
        }
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    }
}

pub async fn auth_user_sign_in_router(
    State(state): State<AppState>,
    Json(payload): Json<UserSigninPayload>,
) -> impl IntoResponse {
    let email_str = extract_or_early_return!(payload.email.ok_or("email is required"));

    let password_str = extract_or_early_return!(payload.password.ok_or("Password is required"));

    let email = extract_or_early_return!(Email::new(&email_str));
    let password = extract_or_early_return!(Password::new(&password_str));

    match state
        .auth_user_service
        .sign_in_user(&email, &password)
        .await
    {
        Ok(token) => {
            let success_response = AuthSuccessResponse {
                token,
                message: "success".to_string(),
            };

            (StatusCode::CREATED, Json(success_response)).into_response()
        }
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    }
}
