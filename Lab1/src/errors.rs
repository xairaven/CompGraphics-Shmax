use crate::config::ConfigError;
use crate::io::IoError;
use crate::logs::LogError;
use crate::ui::GraphicsBackendError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("Graphics Backend. {0}")]
    GraphicsBackend(#[from] GraphicsBackendError),

    #[error("I/O. {0}")]
    Io(#[from] IoError),

    #[error("Logger. {0}")]
    Log(#[from] LogError),
}
