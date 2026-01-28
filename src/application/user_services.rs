use std::sync::Arc;

use crate::{
    domain::{
        user::{DirectUsersDetails, Email, Password, Users},
        uuid_lib::Id,
    },
    error::AuthError,
    payload_description::{SignUpUserData, UpdateUser},
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

    pub async fn sign_up_user(&self, data: SignUpUserData) -> Result<String, AuthError> {
        if self.repo.find_by_email(&data.email).await?.is_some() {
            return Err(AuthError::UserExists);
        }

        let user = Users::new(data)?;

        self.repo.create_user(&user).await?;

        Ok(self.jwt.generate(&user.id.as_uuid().to_string()))
    }

    pub async fn sign_in_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<String, AuthError> {
        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if !user.verify_password(password) {
            return Err(AuthError::PasswordDoesNotMatchError(
                password.as_str().to_string(),
            ));
        }

        Ok(self.jwt.generate(&user.id.as_uuid().to_string()))
    }

    pub async fn find_all_users(&self) -> Result<Vec<DirectUsersDetails>, AuthError> {
        let user_data = self.repo.find_all_users().await?;

        Ok(user_data)
    }

    pub async fn find_single_user(
        &self,
        email: &Email,
    ) -> Result<Option<DirectUsersDetails>, AuthError> {
        let user = self
            .repo
            .find_by_email(email)
            .await?
            .ok_or(AuthError::UserNotFound)?;

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

    pub async fn delete_user(&self, user_id: &Id) -> Result<bool, AuthError> {
        let user_data = self.repo.delete_user(user_id).await?;

        if !user_data {
            return Err(AuthError::UserNotFound);
        }

        Ok(true)
    }

    pub async fn update_user(&self, users: UpdateUser) -> Result<bool, AuthError> {
        if self.repo.find_by_id(&users.id).await?.is_none() {
            return Err(AuthError::UserNotFound);
        }

        let user_data = self.repo.update_user(users).await?;

        if !user_data {
            return Err(AuthError::UserNotFound);
        }

        Ok(true)
    }
}
