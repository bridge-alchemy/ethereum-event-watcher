use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WatcherError {
    #[error("SysError {0} ")] SysError(String),
    #[error("RequestError {0}")] RequestError(String),
    #[error("IoError {0}")] IoError(String),
}

impl WatcherError {
    pub fn catch_from<E>(e: E) -> Self where E: std::error::Error {
        Self::SysError(format!("sys error: {:?}", e.to_string()))
    }
}

impl From<std::io::Error> for WatcherError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

impl From<reqwest::Error> for WatcherError {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e.to_string())
    }
}