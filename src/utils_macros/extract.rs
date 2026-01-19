#[macro_export]
macro_rules! extract_or_early_return {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let error_response = ErrorResponse {
                    message: e.to_string(),
                };

                return (StatusCode::BAD_REQUEST, Json(error_response)).into_response();
            }
        }
    };
    ($expr:expr, $status:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                let error_response = ErrorResponse {
                    message: e.to_string(),
                };

                return ($status, Json(error_response)).into_response();
            }
        }
    };

    // Allow overriding with custom message + optional status
    ($expr:expr, $status:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => {
                let error_response = ErrorResponse {
                    message: $msg.to_string(),
                };
                return ($status, Json(error_response)).into_response();
            }
        }
    };
}
