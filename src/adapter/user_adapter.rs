use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::user::{DirectUsersDetails, Email, Name, Password, PhoneNumber, Roles, UserId, Users},
    error::AuthError,
    payload_description::UpdateUser,
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
    async fn create_user(&self, user: &Users) -> Result<(), AuthError> {
        let _ = sqlx::query!(
            r#"
                INSERT INTO users (
                    id, email, name, phone_number, password, roles,
                    created_by, created_by_name, created_by_email,
                    edited_by, edited_by_name, edited_by_email,
                    created_at, updated_at
                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)
            "#,
            user.id.as_uuid(),
            user.email.as_str(),
            user.name.as_str(),
            user.phone_number.as_ref().map(|p| p.as_str()),
            user.password.as_ref().map(|p| p),
            user.roles.as_str(),
            user.created_by.as_ref().map(|c| c.as_uuid()),
            user.created_by_name.as_ref().map(|n| n.as_str()),
            user.created_by_email.as_ref().map(|e| e.as_str()),
            user.edited_by.as_ref().map(|c| c.as_uuid()),
            user.edited_by_name.as_ref().map(|n| n.as_str()),
            user.edited_by_email.as_ref().map(|e| e.as_str()),
            user.created_at,
            user.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<Users>, AuthError> {
        let row = sqlx::query!(
            r#"SELECT id, email, name, phone_number, roles, password, created_at, updated_at FROM users WHERE email=$1"#,
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        Ok(row
            .map(|d| {
                Ok(Users {
                    id: UserId(d.id),
                    email: Email(d.email),
                    name: Name(d.name),
                    phone_number: d.phone_number.map(PhoneNumber),
                    roles: Roles::new(&d.roles)?,
                    password: Some(d.password),
                    created_by: None,
                    created_by_name: None,
                    created_by_email: None,
                    edited_by: None,
                    edited_by_name: None,
                    edited_by_email: None,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<Users>, AuthError> {
        let row = sqlx::query!(
            r#"SELECT id, email, name, phone_number, roles, password, created_at, updated_at FROM users WHERE id=$1"#,
            user_id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        Ok(row
            .map(|d| {
                Ok(Users {
                    id: UserId(d.id),
                    email: Email(d.email),
                    name: Name(d.name),
                    phone_number: d.phone_number.map(PhoneNumber),
                    roles: Roles::new(&d.roles)?,
                    password: Some(d.password),
                    created_by: None,
                    created_by_name: None,
                    created_by_email: None,
                    edited_by: None,
                    edited_by_name: None,
                    edited_by_email: None,
                    created_at: d.created_at,
                    updated_at: d.updated_at,
                })
            })
            .transpose()?)
    }

    async fn delete_user(&self, user_id: &UserId) -> Result<bool, AuthError> {
        let result = sqlx::query!("DELETE FROM users where id = $1", user_id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|err| AuthError::DatabaseError(err.to_string()))?;

        let deleted = result.rows_affected() > 0;

        Ok(deleted)
    }

    async fn update_user(&self, users: UpdateUser) -> Result<bool, AuthError> {
        let result = sqlx::query!(
            r#"
        UPDATE users
        SET email = COALESCE($1, email),
            name = COALESCE($2, name),
            phone_number = COALESCE($3, phone_number),
            password = COALESCE($4, password),
            roles = COALESCE($5, roles),
            edited_by = $6,
            edited_by_name = $7,
            edited_by_email = $8,
            updated_at = NOW()
        WHERE id = $9
        "#,
            users.email.as_ref().map(|e| e.as_str()),
            users.name.as_ref().map(|n| n.as_str()),
            users.phone_number.as_ref().map(|p| p.as_str()),
            users.password.as_ref().map(|p| p.as_str()),
            users.roles.as_ref().map(|r| r.as_str()),
            users.edited_by,
            users.edited_by_name,
            users.edited_by_email,
            users.id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_all_users(&self) -> Result<Vec<DirectUsersDetails>, AuthError> {
        let rows = sqlx::query!(
            r#"SELECT id, email, name, phone_number, roles, created_at, updated_at FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        let users = rows
            .into_iter()
            .map(|user| {
                Ok(DirectUsersDetails {
                    id: UserId(user.id),
                    email: Email(user.email),
                    name: Name(user.name),
                    roles: Roles::new(&user.roles)?,
                    phone_number: user.phone_number.map(PhoneNumber),
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            })
            .collect();

        users
    }
}
