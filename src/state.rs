use std::sync::Arc;

use crate::{
    application::{
        AuthUserServices, StackServices, blog_services::BlogServices,
        image_upload_services::ImageUploadService, project_services::ProjectServices,
        refresh_token_services::RefreshTokenServices,
    },
    port::{UserDBServices, jwt::JwtService},
};

#[derive(Clone)]
pub struct AppState {
    pub auth_user_service: Arc<AuthUserServices>,
    pub stack_services: Arc<StackServices>,
    pub blog_services: Arc<BlogServices>,
    pub project_services: Arc<ProjectServices>,
    pub jwt_services: Arc<dyn JwtService + Send + Sync>,
    pub(crate) image_service: Arc<ImageUploadService>,
    pub refresh_token_service: Arc<RefreshTokenServices>,
    pub user_repo: Arc<dyn UserDBServices + Send + Sync>,
}
