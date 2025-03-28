/// A handy type alias for `Result<T, crate::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Enumeration of errors that can occur in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// File not found error triggered by [`std::io::Error`]
    #[error("No such file or directory: {0}")]
    FileNotFound(String),
    /// Errors passed through from [`serde_json::Error`]
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    /// Errors passed through from [`meilisearch_sdk::errors::Error`]
    #[error(transparent)]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),
}
