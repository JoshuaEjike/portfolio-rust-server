use crate::{
    error::api_error::ApiErrors,
    state::AppState,
    utils::cookies::{clear_refresh_cookies, set_refresh_cookie},
};

use axum::{Json, extract::State};
use tower_cookies::Cookies;

pub async fn refresh(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<Json<serde_json::Value>, ApiErrors> {
    let refresh = cookies
        .get("refresh_token")
        .ok_or(ApiErrors::Unauthorized("Missing refresh token".into()))?
        .value()
        .to_string();

    let tokens = state.refresh_token_service.handle_refresh(refresh).await?;

    cookies.add(set_refresh_cookie(tokens.refresh_token));

    Ok(Json(
        serde_json::json!({ "access_token": tokens.access_token }),
    ))
}

pub async fn logout(State(state): State<AppState>, cookies: Cookies) -> Result<(), ApiErrors> {
    if let Some(cookie) = cookies.get("refresh_token") {
        let token = cookie.value().into();

        state.refresh_token_service.handle_logout(token).await?;
    }

    cookies.remove(clear_refresh_cookies());

    Ok(())
}
