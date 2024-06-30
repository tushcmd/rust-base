//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum  Error {
    /// For starter, to remove as code matures.
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}