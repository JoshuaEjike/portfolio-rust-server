use std::sync::Arc;

use uuid::Uuid;

use crate::{
    core::password_core::{hashing_password, verify_password},
    domain::user::{DirectUsersDetails, Users},
    error::api_error::ApiErrors,
    fields::{Email, Password},
    payload_description::{
        SignUpUserData, UpdateUser, user_payload_description::UpdateUserDetails,
    },
    port::{UserDBServices, jwt::JwtService},
};

pub struct AuthUserServices {
    repo: Arc<dyn UserDBServices + Send + Sync>,
    jwt: Arc<dyn JwtService + Send + Sync>,
}

impl AuthUserServices {
    pub fn new(
        repo: Arc<dyn UserDBServices + Send + Sync>,
        jwt: Arc<dyn JwtService + Send + Sync>,
    ) -> Self {
        Self { repo, jwt }
    }

    pub async fn sign_up_user(&self, data: SignUpUserData) -> Result<String, ApiErrors> {
        if self.repo.find_by_email(&data.email).await?.is_some() {
            return Err(ApiErrors::NotFound("user does not exist".to_string()));
        }

        let user = Users::new(data)?;

        self.repo.create_user(&user).await?;

        Ok(self.jwt.generate(&user.id.to_string()))
    }

    pub async fn sign_in_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<String, ApiErrors> {
        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or(ApiErrors::NotFound("User not found".to_string()))?;

        if !verify_password(user.password, password.as_str().to_string()) {
            return Err(ApiErrors::PasswordFail(password.as_str().to_string()));
        }

        Ok(self.jwt.generate(&user.id.to_string()))
    }

    pub async fn find_all_users(&self) -> Result<Vec<DirectUsersDetails>, ApiErrors> {
        let user_data = self.repo.find_all_users().await?;

        Ok(user_data)
    }

    pub async fn find_single_user(
        &self,
        email: &Email,
    ) -> Result<Option<DirectUsersDetails>, ApiErrors> {
        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or(ApiErrors::NotFound("User not found".to_string()))?;

        Ok(Some(DirectUsersDetails {
            id: user.id,
            name: user.name,
            email: user.email,
            phone_number: user.phone_number,
            roles: user.roles,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }))
    }

    pub async fn delete_user(&self, user_id: &Uuid) -> Result<bool, ApiErrors> {
        let user_data = self.repo.delete_user(user_id).await?;

        if !user_data {
            return Err(ApiErrors::NotFound("User not found".to_string()));
        }

        Ok(true)
    }

    pub async fn update_user(&self, users: UpdateUser) -> Result<bool, ApiErrors> {
        if self.repo.find_by_id(&users.id).await?.is_none() {
            return Err(ApiErrors::NotFound("User not found".to_string()));
        }

        let hashed_password = if let Some(password) = users.password {
            Some(hashing_password(password.as_str().to_string())?)
        } else {
            None
        };

        let details = UpdateUserDetails {
            id: users.id,
            name: users.name,
            phone_number: users.phone_number,
            password: hashed_password,
            roles: users.roles,
            edited_by: users.edited_by,
            edited_by_name: users.edited_by_name,
            edited_by_email: users.edited_by_email,
        };

        let user_data = self.repo.update_user(details).await?;

        if !user_data {
            return Err(ApiErrors::NotFound("User not found".to_string()));
        }

        Ok(true)
    }
}
