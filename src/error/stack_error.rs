use thiserror::Error;

#[derive(Debug, Error)]
pub enum StackError {
    #[error("Database Error:{0}")]
    DatabaseError(String),

    #[error("Stack Already Exist")]
    StackExists,
    #[error("Stack Not Found")]
    StackNotFound,
}
