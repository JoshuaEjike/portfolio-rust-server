use std::sync::Arc;

use crate::{
    application::{AuthUserServices, StackServices},
    port::{UserDBServices, jwt::JwtService},
};

#[derive(Clone)]
pub struct AppState {
    pub auth_user_service: Arc<AuthUserServices>,
    pub stack_services: Arc<StackServices>,
    pub jwt_services: Arc<dyn JwtService + Send + Sync>,
    pub user_repo: Arc<dyn UserDBServices + Send + Sync>,
}
