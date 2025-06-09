/*
 * config/v1.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: u32,
    pub placeholder: String,
}

impl Config {
    pub fn new(placeholder: &str) -> Config {
        Config {
            version: 1,
            placeholder: placeholder.to_string(),
        }
    }
}
