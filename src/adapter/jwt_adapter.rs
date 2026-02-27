use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{core::jwt_core::generate_token, error::api_error::ApiErrors, port::jwt::JwtService};

use jsonwebtoken::{DecodingKey, Validation, decode};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct JwtServiceImpl {
    secret: String,
    expiry_second: u64,
}

impl JwtServiceImpl {
    pub fn new(secret: String, expiry_second: u64) -> Self {
        Self {
            secret,
            expiry_second,
        }
    }
}

impl JwtService for JwtServiceImpl {
    fn generate(&self, user_id: Uuid) -> Result<String, ApiErrors> {
        let token = generate_token(user_id, &self.secret, self.expiry_second)?;

        Ok(token)
    }

    fn verify(&self, token: &str) -> Option<String> {
        // decode::<Claims>(
        //     &token,
        //     &DecodingKey::from_secret(self.secret.as_ref()),
        //     &Validation::default(),
        // )
        // .map(|data| data.claims.sub)
        // .ok()

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(data) => {
                println!("✅ Decoded claims: {:?}", data.claims);
                Some(data.claims.sub)
            }
            Err(err) => {
                println!("❌ JWT decode error: {err:?}");
                None
            }
        }
    }
}
