use thiserror::Error;

#[derive(Error, Debug)]
pub enum HikvError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found value for key: {0}")]
    NotFound(String),
}
