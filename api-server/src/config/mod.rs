/*
 * config/mod.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

use serde::Deserialize;
use std::{env, path::PathBuf};
use thiserror::Error;
use tokio::{fs, io};

mod v1;

// Update here on config version increment.
pub mod current_schema {
    pub use super::v1::schema::*;
}

use current_schema::*;

#[derive(Deserialize)]
struct ConfigVersion {
    pub version: u32,
}

pub async fn read_config() -> Result<Config, ConfigReadError> {
    let file_path: PathBuf = match env::var("CONF_FILE") {
        Ok(x) => PathBuf::from(x),
        Err(_) => PathBuf::from("config.toml"),
    };

    if file_path.exists() {
        let config_contents = fs::read_to_string(file_path).await?;

        let config_ver: ConfigVersion = toml::from_str(&config_contents)?;

        // Update here on config version increment.
        match config_ver.version {
            1 => Ok(toml::from_str(&config_contents)?),
            _ => Err(ConfigReadError::Version),
        }
    } else {
        let default_config = Config::default();

        let default_str = toml::to_string(&default_config)?;

        fs::write(file_path, default_str).await?;

        Ok(default_config)
    }
}

#[allow(dead_code)]
fn adapt_version() {
    // NOT IMPLEMENTED YET
}

#[derive(Error, Debug)]
pub enum ConfigReadError {
    #[error("Failed to read file")]
    Io(#[from] io::Error),
    #[error("Invalid version. This version is not known to this program.")]
    Version,
    #[error("Unable to parse configuration file.")]
    Deserialize(#[from] toml::de::Error),
    #[error("Unable to serialize config.")]
    Serialize(#[from] toml::ser::Error),
}
