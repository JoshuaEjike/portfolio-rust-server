use axum::{
    Json,
    body::Body,
    extract::OriginalUri,
    http::{Method, Response, StatusCode},
    response::IntoResponse,
};

use crate::payload_description::ErrorResponse;

// pub fn error_manager(status: StatusCode, msg: impl Into<String>) -> impl IntoResponse {
//     (status, Json(ErrorResponse { message: msg.into() }))
// }

pub fn error_manager(status: StatusCode, msg: impl Into<String>) -> Response<Body> {
    (
        status,
        Json(ErrorResponse {
            message: msg.into(),
        }),
    )
        .into_response()
}

// pub async fn handle_404() -> impl IntoResponse {
//     println!("does not exist");
//     let body = ErrorResponse {
//         message: "route does not exist".to_string(),
//     };

//     (StatusCode::NOT_FOUND, Json(body)).into_response()
// }

pub async fn handle_404_with_path(method: Method, uri: OriginalUri) -> impl IntoResponse {
    let body = ErrorResponse {
        message: format!("Route {} {} does not exist", method, uri.0),
    };
    (StatusCode::NOT_FOUND, Json(body))
}
