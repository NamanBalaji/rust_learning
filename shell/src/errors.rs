use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("{0}: command not found")]
    CommandNotFound(String),
}
