use std::sync::Arc;

use crate::{
    domain::user::{Email, Name, Password, PhoneNumber, Roles, UserId, Users},
    error::AuthError,
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

    pub async fn sign_up_user(
        &self,
        name: Name,
        email: Email,
        phone_number: Option<PhoneNumber>,
        roles: Roles,
        password: Password,
        created_by: Option<UserId>,
        created_by_name: Option<Name>,
        created_by_email: Option<Email>,
    ) -> Result<String, AuthError> {
        if self.repo.find_by_email(&email).await?.is_some() {
            return Err(AuthError::UserExists);
        }

        let user = Users::new(
            name,
            email,
            phone_number,
            roles,
            password,
            created_by,
            created_by_name,
            created_by_email,
        )?;

        let _ = self.repo.create_user(&user).await?;

        Ok(self.jwt.generate(&user.id.as_uuid().to_string()))
    }

    pub async fn sign_in_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<String, AuthError> {
        let user = self
            .repo
            .find_by_email(&email)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        // let password_state = user.password.as_deref().ok_or(AuthError::MissingPassword)?;

        println!("{0}:{1:?}", user.verify_password(&password), password);

        if !user.verify_password(&password) {
            return Err(AuthError::PasswordDoesNotMatchError(
                password.as_str().to_string(),
            ));
        }

        Ok(self.jwt.generate(&user.id.as_uuid().to_string()))
    }
}
