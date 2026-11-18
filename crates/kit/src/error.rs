#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum KitError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, KitError>;
