mod adapter;
mod api;
mod application;
mod config;
mod core;
mod domain;
mod error;
mod extractor;
mod fields;
mod payload_description;
mod payload_handler;
mod port;
mod router_handler;
mod state;
mod utils;
mod utils_macros;

use std::{net::SocketAddr, sync::Arc};

use config::Config;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::{
    adapter::{
        JwtServiceImpl, PostgreStackRepository, PostgreUserRepository,
        blog_adapter::PostgreBlogRepository, cloudinary::CloudinaryUploader,
        project_adapter::PostgreProjectRepository,
        refresh_token_adapter::PostgreRefreshTokenRepository,
    },
    api::app_apis,
    application::{
        AuthUserServices, StackServices, blog_services::BlogServices,
        image_upload_services::ImageUploadService, project_services::ProjectServices,
        refresh_token_services::RefreshTokenServices,
    },
    port::{
        StackDBServices, UserDBServices, blog_db::BlogDBServices, project_db::ProjectDBServices,
        refresh_token_db::RefreshTokenDBServices,
    },
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_max_connections.unwrap_or(12))
        .connect(&config.database_url)
        .await?;

    let uploader = Arc::new(CloudinaryUploader::new(
        config.cloud_name,
        config.cloud_api_key,
        config.cloud_api_secret,
    ));

    let user_repo =
        Arc::new(PostgreUserRepository::new(pool.clone())) as Arc<dyn UserDBServices + Send + Sync>;

    let jwt_services = Arc::new(JwtServiceImpl::new(
        config.jwt_secret.clone(),
        config.jwt_expiry_seconds,
    ));

    let stack_repo = Arc::new(PostgreStackRepository::new(pool.clone()))
        as Arc<dyn StackDBServices + Send + Sync>;

    let blog_repo =
        Arc::new(PostgreBlogRepository::new(pool.clone())) as Arc<dyn BlogDBServices + Send + Sync>;

    let project_repo = Arc::new(PostgreProjectRepository::new(pool.clone()))
        as Arc<dyn ProjectDBServices + Send + Sync>;

    let refresh_token_repo = Arc::new(PostgreRefreshTokenRepository::new(pool.clone()))
        as Arc<dyn RefreshTokenDBServices + Send + Sync>;

    let auth_user_service = Arc::new(AuthUserServices::new(
        user_repo.clone(),
        jwt_services.clone(),
    ));

    let stack_services = Arc::new(StackServices::new(stack_repo));

    let blog_services = Arc::new(BlogServices::new(blog_repo));

    let project_services = Arc::new(ProjectServices::new(project_repo));

    let image_service = Arc::new(ImageUploadService::new(uploader.clone()));

    let refresh_token_service = Arc::new(RefreshTokenServices::new(
        refresh_token_repo.clone(),
        config.jwt_secret.clone(),
        config.jwt_expiry_seconds,
    ));

    let app_state = AppState {
        auth_user_service,
        stack_services,
        blog_services,
        project_services,
        jwt_services,
        image_service,
        refresh_token_service,
        user_repo,
    };

    let app = app_apis(app_state);

    let port = config.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Server running at http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
