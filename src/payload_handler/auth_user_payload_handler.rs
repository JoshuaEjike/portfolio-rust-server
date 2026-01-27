// use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};

// use crate::payload_description::ErrorResponse;

// pub fn auth_user_json_payload_handler<T>(
//     payload: Result<Json<T>, JsonRejection>,
// ) -> Result<T, impl IntoResponse>
// where
//     T: serde::de::DeserializeOwned,
// {
//     match payload {
//         Ok(p) => Ok(p.0),
//         Err(err) => {
//             let error_response = ErrorResponse {
//                 message: match err {
//                     JsonRejection::JsonDataError(e) => {
//                         let err_str = e.to_string();

//                         if err_str.contains("password") {
//                             "Password is required".to_string()
//                         } else if err_str.contains("name") {
//                             "Name is required".to_string()
//                         } else if err_str.contains("email") {
//                             "Email is required".to_string()
//                         } else if err_str.contains("phone_number") {
//                             "Phone Number is required".to_string()
//                         } else if err_str.contains("roles") {
//                             "Roles is required".to_string()
//                         } else {
//                             format!("Invalid request: {}", e)
//                         }
//                     }
//                     _ => "Invalid JSON body".to_string(),
//                 },
//             };

//             Err((StatusCode::BAD_REQUEST, Json(error_response)).into_response())
//         }
//     }
// }
