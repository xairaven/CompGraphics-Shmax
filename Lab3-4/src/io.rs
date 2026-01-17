use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to get current directory: {0}")]
    CurrentDirectory(std::io::Error),

    #[error("Failed to create parent directories. {0}")]
    ParentDirectoriesCreation(std::io::Error),
}
