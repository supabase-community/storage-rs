use std::env;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Environment Variable Unreadable")]
    InvalidEnvironmentVariable(#[from] env::VarError),
}
