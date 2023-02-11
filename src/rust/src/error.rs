use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnextendrError {
    #[error("Unexpected type: {0}")]
    UnexpectedType(String),
    #[error("Unknown error happened")]
    Unknown,
}
