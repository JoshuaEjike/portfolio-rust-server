use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::port::jwt::JwtService;

use jsonwebtoken::{DecodingKey, Validation, decode};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    exp: usize,
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
    fn generate(&self, user_id: &str) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(self.expiry_second as i64))
            .expect("valid timespame")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .expect("Failed to generate token")
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
