use crate::errors::ProjectError;
use crate::io::IoError;
use crate::logs::LogLevel;
use crate::ui::themes::Theme;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub log_level: LogLevel,
    pub theme: Theme,
}

impl Config {
    const FILENAME: &str = "config.toml";

    fn path() -> Result<PathBuf, IoError> {
        let mut current_dir =
            std::env::current_exe().map_err(IoError::CurrentDirectory)?;
        current_dir.pop(); // Remove executable name

        std::fs::create_dir_all(&current_dir)
            .map_err(IoError::ParentDirectoriesCreation)?;

        Ok(current_dir.join(Self::FILENAME))
    }

    pub fn from_file() -> Result<Self, ProjectError> {
        match Self::path() {
            Ok(path) => {
                let text = std::fs::read_to_string(&path);
                match text {
                    Ok(text) => {
                        let config: Config = toml::from_str(&text)
                            .map_err(ConfigError::Deserialization)?;
                        Ok(config)
                    },
                    Err(_) => {
                        let config = Config::default();
                        config.save_to_file()?;
                        Ok(config)
                    },
                }
            },
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn save_to_file(&self) -> Result<(), ProjectError> {
        let data = toml::to_string(&self).map_err(ConfigError::Serialization)?;
        let path = Self::path()?;

        std::fs::write(path, data).map_err(ConfigError::Write)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to serialize. {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Failed to deserialize. {0}")]
    Deserialization(#[from] toml::de::Error),

    #[error("Failed to write to file. {0}")]
    Write(std::io::Error),
}
