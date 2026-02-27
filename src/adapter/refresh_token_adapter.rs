use async_trait::async_trait;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::refresh_token::RefreshTokenRecord, error::api_error::ApiErrors,
    port::refresh_token_db::RefreshTokenDBServices,
};

pub struct PostgreRefreshTokenRepository {
    pool: PgPool,
}

impl PostgreRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenDBServices for PostgreRefreshTokenRepository {
    async fn store_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), ApiErrors> {
        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (id, user_id, token, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            user_id,
            token,
            Utc::now().naive_utc() + Duration::days(7),
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to store refresh token".into()))?;

        Ok(())
    }

    async fn find_refresh_token(
        &self,
        token: &str,
    ) -> Result<Option<RefreshTokenRecord>, ApiErrors> {
        let rec = sqlx::query_as!(
            RefreshTokenRecord,
            r#"
            SELECT id, user_id, expires_at, revoked
            FROM refresh_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Refresh token lookup failed".into()))?;

        Ok(rec)

        //     let rec = sqlx::query!(
        //         r#"
        //         SELECT id, user_id, token, expires_at, revoked
        //         FROM refresh_tokens
        //         WHERE token = $1
        //         "#,
        //         token
        //     )
        //     .fetch_optional(&self.pool)
        //     .await
        //     .map_err(|_| ApiErrors::InternalServerError("Refresh token lookup failed".into()))?;

        //     Ok(RefreshTokenRecord{
        //         id: rec.id,
        // user_id: rec.user_id,
        // token: rec.token,
        // expires_at: rec.expires_at,
        // revoked: rec.revoked,
        //     })
    }

    async fn revoke_refresh_token(&self, id: Uuid) -> Result<(), ApiErrors> {
        sqlx::query!(
            r#"UPDATE refresh_tokens SET revoked = true WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Token revoke failed".into()))?;

        Ok(())
    }

    async fn revoke_refresh_token_by_value(&self, token: &str) -> Result<(), ApiErrors> {
        sqlx::query!(
            r#"UPDATE refresh_tokens SET revoked = true WHERE token = $1"#,
            token
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Token revoke failed".into()))?;

        Ok(())
    }
}
