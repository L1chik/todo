use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("Invalid Token {0}")]
    InvalidToken(String),
}