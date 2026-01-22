pub mod auth_error;
pub mod general_error;

pub use auth_error::AuthError;
pub use general_error::error_manager;
pub use general_error::handle_404;
pub use general_error::handle_404_with_path;
