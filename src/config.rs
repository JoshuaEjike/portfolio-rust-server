use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub jwt_expiry_seconds: u64,
    pub db_pool_max_connections: Option<u32>,
    // pub cloud_name: String,
    // pub cloud_api_key: String,
    // pub cloud_api_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "9400".to_string())
                .parse::<u16>() // ✅ parse as u16
                .expect("PORT must be a valid u16"),
            jwt_expiry_seconds: env::var("JWT_EXPIRY_SECONDS")
                .expect("JWT_EXPIRY_SECONDS must be set")
                .parse()
                .expect("JWT_EXPIRY_SECONDS must be a number"),
            db_pool_max_connections: env::var("DB_POOL_MAX_CONNECTIONS").ok().map(|s| {
                s.parse::<u32>()
                    .expect("DB_POOL_MAX_CONNECTIONS must be a number")
            }),
            // cloud_name: env::var("CLOUD_NAME").expect("CLOUD_NAME must be set"),
            // cloud_api_key: env::var("CLOUD_API_KEY").expect("CLOUD_API_KEY must be set"),
            // cloud_api_secret: env::var("CLOUD_API_SECRET").expect("CLOUD_API_SECRET must be set"),
        }
    }
}
