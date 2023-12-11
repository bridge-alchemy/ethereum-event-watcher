use crate::errors::WatcherError;

pub type ServerResult<T> = std::result::Result<T, WatcherError>;

#[derive(Debug)]
pub struct AppState {
    pub app_name: String,
}
