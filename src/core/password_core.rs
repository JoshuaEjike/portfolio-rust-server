use bcrypt::{DEFAULT_COST, hash, verify};

use crate::error::api_error::ApiErrors;

pub fn hashing_password(password: String) -> Result<String, ApiErrors> {
    let result = hash(password.as_str(), DEFAULT_COST)
        .map_err(|_| ApiErrors::PasswordFail("Invalid password hash".to_string()))?;

    Ok(result)
}

pub fn verify_password(hash: String, password: String) -> bool {
    let result = verify(password.as_str(), hash.as_str()).unwrap_or(false);

    result
}
