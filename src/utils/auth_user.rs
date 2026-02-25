// use crate::{
//     domain::user::{DirectUsersDetails, Users},
//     error::api_error::ApiErrors,
//     port::{UserDBServices, jwt::JwtService},
// };

// use axum::http::HeaderMap;
// use std::sync::Arc;
// use uuid::Uuid;

// #[derive(Debug, Clone)]
// pub struct CurrentUser(pub DirectUsersDetails);

// impl CurrentUser {
//     pub async fn from_headers(
//         headers: &HeaderMap,
//         jwt_service: Arc<dyn JwtService>,
//         user_repo: Arc<dyn UserDBServices>,
//     ) -> Result<Self, ApiErrors> {
//         // 🔐 Extract Authorization header
//         let auth_header = headers
//             .get("Authorization")
//             .and_then(|h| h.to_str().ok())
//             .ok_or(ApiErrors::Unauthorized("unauthorized".to_string()))?;

//         let token = auth_header
//             .strip_prefix("Bearer ")
//             .ok_or(ApiErrors::Unauthorized("unauthorized".to_string()))?;

//         // ✅ Verify token → get user_id as String
//         let user_id_str = jwt_service.verify(token).ok_or(ApiErrors::Unauthorized(
//             "❌ JWT decode error: {err:?}".to_string(),
//         ))?; // Option<String> → String

//         // ✅ Convert into Id (your domain type)
//         let user_id =
//             Uuid::parse_str(&user_id_str).map_err(|e| ApiErrors::InvalidString(e.to_string()))?;
//         // ^ make sure you have a constructor for this (e.g. new(String))

//         // ✅ Look up user in database
//         let user = user_repo
//             .find_by_id(&user_id)
//             .await?
//             .ok_or(ApiErrors::Unauthorized("User not found".to_string()))?;

//         Ok(CurrentUser(user))
//     }
// }
