use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub enum CommandError {
    #[error("Docker API error: {0}")]
    DockerError(String),
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}