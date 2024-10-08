use std::env;

use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Operation failed with status: {status}: {message}")]
    StorageError { status: StatusCode, message: String },
    #[error("Environment Variable Unreadable")]
    InvalidEnvironmentVariable(#[from] env::VarError),
    #[error("Failed to Serialize or Deserialize")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("Header Value is Invalid")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Failed to send request")]
    RequestError(#[from] reqwest::Error),
}
