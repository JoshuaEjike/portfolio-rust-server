use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::payload_description::ErrorResponse;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User Already Exist")]
    UserExists,
    #[error("User Not Found")]
    UserNotFound,

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Forbidden: {0}")]
    Internal(String),

    #[error("{0} is not a valid email address")]
    EmailValidationError(String),
    #[error("Full Name must only contain alphabet and some special charaters.")]
    NameMustBeAlphabetic,
    #[error("password too short (min 8 characters).")]
    TooShort,
    #[error("password musht contain at least one uppercase.")]
    MissingUppercase,
    #[error("password must conatin at least one number.")]
    MissingNumber,
    #[error("password must contain at least one special char.")]
    MissingSpecialChar,
    #[error("root, mid, normal are the roles attached to the Admin roles.")]
    AdminRolesError,

    #[error("Phone number is too short. Expected a valid international phone number.")]
    PhoneNumberTooShort,

    #[error("Phone number exceeds the maximum allowed length for international numbers.")]
    PhoneNumberTooLong,

    #[error(
        "Invalid phone number format. Expected an international phone number in E.164 format (e.g., +2348012345678)."
    )]
    InvalidPhoneNumberFormat,

    #[error("Hash error")]
    HashError,

    #[error("Database Error:{0}")]
    DatabaseError(String),

    #[error("password '{0}' does not match")]
    PasswordDoesNotMatchError(String),

    #[error("password is missing")]
    MissingPassword,

    #[error("missing authorization header")]
    MissingAuthHeader,
    #[error("invalid authorization scheme")]
    InvalidScheme,
    #[error("invalid token")]
    InvalidToken,
    #[error("unauthorized")]
    Unauthorized,
    #[error("this is not a valid uuid or string")]
    InvalidString(uuid::Error),
}

// impl IntoResponse for AuthError {
//     fn into_response(self) -> Response {
//         let (status, message) = match &self {
//             AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
//             AuthError::Forbidden(_) => (StatusCode::FORBIDDEN, self.to_string()),
//             AuthError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
//             AuthError::DatabaseError(_) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 self.to_string(),
//             ),
//             AuthError::Internal(_) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 self.to_string(),
//             ),
//             AuthError::UserExists => todo!(),
// AuthError::EmailValidationError(_) => todo!(),
//             AuthError::NameMustBeAlphabetic => todo!(),
//             AuthError::TooShort => todo!(),
//             AuthError::MissingUppercase => todo!(),
//             AuthError::MissingNumber => todo!(),
//             AuthError::MissingSpecialChar => todo!(),
//             AuthError::AdminRolesError => todo!(),
//             AuthError::PhoneNumberTooShort => todo!(),
//             AuthError::PhoneNumberTooLong => todo!(),
//             AuthError::InvalidPhoneNumberFormat => todo!(),
//             AuthError::HashError => todo!(),
//             AuthError::PasswordDoesNotMatchError(_) => todo!(),
//             AuthError::MissingPassword => todo!(),
//             AuthError::MissingAuthHeader => todo!(),
//             AuthError::InvalidScheme => todo!(),
//             AuthError::InvalidToken => todo!(),
//             AuthError::InvalidString(error) => todo!(),
//         };

//         (status, Json(ErrorResponse { message })).into_response()
//     }
// }
