use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, Error>;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde json error {0}")]
    SerdeJsonError(serde_json::Error),
    #[error("internal error occurred. Please try again later.")]
    InternalError {
        #[source]
        source: anyhow::Error,
    },
}
