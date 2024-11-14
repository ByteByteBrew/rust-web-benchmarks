use thiserror::Error;

#[derive(Debug, Error)]
pub enum BenchmarkError {
    #[error("Framework error: {0}")]
    Framework(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<String> for BenchmarkError {
    fn from(msg: String) -> Self {
        BenchmarkError::Other(msg)
    }
}
