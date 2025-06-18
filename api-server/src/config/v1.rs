/*
 * config/v1.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

pub mod schema {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Copy, Clone)]
    pub struct UnixConfig {
        pub unix_socket: bool,
    }

    #[derive(Deserialize, Serialize, Copy, Clone)]
    pub struct Config {
        pub version: u32,
        pub unix: UnixConfig,
    }
}
