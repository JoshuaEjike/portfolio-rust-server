use std::sync::Arc;

use tower_cookies::Cookies;
use uuid::Uuid;

use crate::{
    application::refresh_token_services::RefreshTokenServices, domain::refresh_token::TokenPair,
    error::api_error::ApiErrors, utils::cookies::set_refresh_cookie,
};

pub async fn login_token_core(
    refresh_token_service: Arc<RefreshTokenServices>,
    cookies: Cookies,
    user_id: Uuid,
) -> Result<TokenPair, ApiErrors> {
    let tokens = refresh_token_service.handle_login(user_id).await?;

    cookies.add(set_refresh_cookie(tokens.refresh_token.clone()));

    Ok(tokens)
}
