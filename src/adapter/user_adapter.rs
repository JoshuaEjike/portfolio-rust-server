use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::user::{DirectUsersAccess, DirectUsersDetails, Users},
    error::api_error::ApiErrors,
    fields::{Email, PhoneNumber, Roles, Text},
    payload_description::user_payload_description::UpdateUserDetails,
    port::UserDBServices,
};

pub struct PostgreUserRepository {
    pool: PgPool,
}

impl PostgreUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserDBServices for PostgreUserRepository {
    async fn create_user(&self, user: &Users) -> Result<(), ApiErrors> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO users (
                    id, email, name, phone_number, password, roles,
                    created_by, created_by_name, created_by_email,
                    created_at, updated_at
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
            "#,
            user.id,
            user.email.as_str(),
            user.name.as_str(),
            user.phone_number.as_ref().map(|p| p.as_str()),
            user.password,
            user.roles.as_str(),
            user.created_by,
            user.created_by_name.as_str(),
            user.created_by_email.as_str(),
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<DirectUsersAccess>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, email, name, phone_number, roles, password, created_at, updated_at FROM users WHERE email=$1"#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::Unauthorized("Invalid credentials".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectUsersAccess {
                    id: d.id,
                    email: Email(d.email),
                    name: Text(d.name),
                    phone_number: d.phone_number.map(PhoneNumber),
                    roles: Roles::new(&d.roles)?,
                    password: d.password,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<DirectUsersDetails>, ApiErrors> {
        let row = sqlx::query!(
            r#"SELECT id, email, name, phone_number, roles, password, created_at, updated_at FROM users WHERE id=$1"#,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ApiErrors::NotFound("User not found".to_string()))?;

        Ok(row
            .map(|d| {
                Ok(DirectUsersDetails {
                    id: d.id,
                    email: Email(d.email),
                    name: Text(d.name),
                    phone_number: d.phone_number.map(PhoneNumber),
                    roles: Roles::new(&d.roles)?,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn delete_user(&self, user_id: &Uuid) -> Result<bool, ApiErrors> {
        let result = sqlx::query!("DELETE FROM users where id = $1", user_id)
            .execute(&self.pool)
            .await
            .map_err(|_| ApiErrors::InternalServerError("Delete failed".to_string()))?;

        let deleted = result.rows_affected() > 0;

        Ok(deleted)
    }

    async fn update_user(&self, users: UpdateUserDetails) -> Result<bool, ApiErrors> {
        let result = sqlx::query!(
            r#"
        UPDATE users
        SET name = COALESCE($1, name),
            phone_number = COALESCE($2, phone_number),
            password = COALESCE($3, password),
            roles = COALESCE($4, roles),
            edited_by = $5,
            edited_by_name = $6,
            edited_by_email = $7,
            updated_at = NOW()
        WHERE id = $8
        "#,
            users.name.as_ref().map(|n| n.as_str()),
            users.phone_number.as_ref().map(|p| p.as_str()),
            users.password,
            users.roles.as_ref().map(|r| r.as_str()),
            users.edited_by,
            users.edited_by_name,
            users.edited_by_email,
            users.id
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Update failed".to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_all_users(&self) -> Result<Vec<DirectUsersDetails>, ApiErrors> {
        let rows = sqlx::query!(
            r#"SELECT id, email, name, phone_number, roles, created_at, updated_at FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ApiErrors::InternalServerError("Failed to fetch users".to_string()))?;

        rows.into_iter()
            .map(|user| {
                Ok(DirectUsersDetails {
                    id: user.id,
                    email: Email(user.email),
                    name: Text(user.name),
                    roles: Roles::new(&user.roles)?,
                    phone_number: user.phone_number.map(PhoneNumber),
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            })
            .collect()
    }
}
