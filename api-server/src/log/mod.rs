/*
 * log/mod.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

use crate::config::current_schema::LoggingMode;
use std::error;
use thiserror::Error;

pub async fn start_logging(logging_mode: Option<LoggingMode>) -> Result<(), LogError> {
    let cargo_pkg_ver = env!(
        "CARGO_PKG_VERSION",
        "Unable to determine build version from environment."
    );
    Ok(())
}

pub async fn failed_init_error<E: error::Error>(e: E) {}

#[derive(Error, Debug)]
pub enum LogError {}
