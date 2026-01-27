use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};

use crate::{
    domain::user::{Email, Name, Password, PhoneNumber, Roles, UserId},
    error::error_manager,
    extract_or_early_return,
    payload_description::{
        AuthSuccessResponse, ErrorResponse, RequestUserEmailPayload, RequestUserIdPayload,
        ResponseForGettingSingleUsersPayload, SignUpUserData, SuccessMessageResponse, UpdateUser,
        UpdateUserPayload, UserSigninPayload, UsersPayloadLoader,
        gobal_response_description::ResponseForGettingUsersPayload,
    },
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

    let user = SignUpUserData {
        name,
        email,
        password,
        phone_number: Some(phone_number),
        roles,
        created_by: Some(current_user.0.id),
        created_by_name: Some(current_user.0.name),
        created_by_email: Some(current_user.0.email),
    };

    match state.auth_user_service.sign_up_user(user).await {
        Ok(token) => {
            let success_response = AuthSuccessResponse {
                token,
                message: "success".to_string(),
            };

            (StatusCode::CREATED, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
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
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

#[axum::debug_handler]
pub async fn get_all_users_router(
    State(state): State<AppState>,
    headers: HeaderMap,
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

    match state.auth_user_service.find_all_users().await {
        Ok(users) => {
            let success_response = ResponseForGettingUsersPayload {
                message: "success".to_string(),
                users,
            };

            // ✅ Works fine in axum 0.8+
            (StatusCode::OK, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}

#[axum::debug_handler]
pub async fn get_delete_user_router(
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

    let user_id = match UserId::from_str(&path.id) {
        Ok(data) => data,
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    };

    match state.auth_user_service.delete_user(&user_id).await {
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

#[axum::debug_handler]
pub async fn get_update_user_router(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(path): Path<RequestUserIdPayload>,
    Json(payload_data): Json<UpdateUserPayload>,
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

    let user_id = match UserId::from_str(&path.id) {
        Ok(data) => data,
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    };

    if let Some(role) = payload_data.roles.as_deref() {
        let role = extract_or_early_return!(Roles::new(role));

        if role.as_str() == "root" {
            return error_manager(
                StatusCode::FORBIDDEN,
                "This admin level can't be updated".to_string(),
            );
        }
    }

    if user.0.roles.as_str() == "normal" {
        return error_manager(
            StatusCode::FORBIDDEN,
            "Becuase of your ADMIN Level you can not update a user.".to_string(),
        );
    }

    let name = payload_data.name.as_deref().and_then(|n| Name::new(n).ok());

    let email = payload_data
        .email
        .as_deref()
        .and_then(|n| Email::new(n).ok());

    let phone_number = payload_data
        .phone_number
        .as_deref()
        .and_then(|n| PhoneNumber::new(n).ok());

    let password = payload_data
        .password
        .as_deref()
        .and_then(|n| Password::new(n).ok());

    let roles = payload_data
        .roles
        .as_deref()
        .and_then(|n| Roles::new(n).ok());

    let updated_product = UpdateUser {
        id: user_id.clone(),
        name,
        email,
        phone_number,
        password,
        roles,
        edited_by: user.0.id.as_uuid(),
        edited_by_name: user.0.email.as_str().to_string(),
        edited_by_email: user.0.name.as_str().to_string(),
    };

    match state.auth_user_service.update_user(updated_product).await {
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

#[axum::debug_handler]
pub async fn get_single_user_router(
    State(state): State<AppState>,
    Path(path): Path<RequestUserEmailPayload>,
) -> impl IntoResponse {
    let email = match Email::new(&path.email) {
        Ok(data) => data,
        Err(err) => {
            return error_manager(StatusCode::BAD_REQUEST, err.to_string());
        }
    };

    match state.auth_user_service.find_single_user(&email).await {
        Ok(users) => {
            let success_response = ResponseForGettingSingleUsersPayload {
                message: "success".to_string(),
                users,
            };

            // ✅ Works fine in axum 0.8+
            (StatusCode::OK, Json(success_response)).into_response()
        }
        Err(err) => error_manager(StatusCode::BAD_REQUEST, err.to_string()),
    }
}
