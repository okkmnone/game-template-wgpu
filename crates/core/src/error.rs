#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;
