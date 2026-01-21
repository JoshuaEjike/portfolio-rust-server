mod adapter;
mod api;
mod application;
mod config;
mod domain;
mod error;
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
    adapter::{JwtServiceImpl, PostgreUserRepository},
    api::app_apis,
    application::AuthUserServices,
    port::UserDBServices,
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(config.db_pool_max_connections.unwrap_or(12))
        .connect(&config.database_url)
        .await?;

    let user_repo =
        Arc::new(PostgreUserRepository::new(pool.clone())) as Arc<dyn UserDBServices + Send + Sync>;

    let jwt_services = Arc::new(JwtServiceImpl::new(
        config.jwt_secret,
        config.jwt_expiry_seconds,
    ));

    let auth_user_service = Arc::new(AuthUserServices::new(
        user_repo.clone(),
        jwt_services.clone(),
    ));

    let app_state = AppState {
        auth_user_service,
        jwt_services,
        user_repo,
    };

    let app = app_apis(app_state);

    let port = config.port;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
