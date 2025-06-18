/*
 * config/default.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

use super::current_schema::*;

pub static CONFIG: Config = Config {
    version: 1,
    unix: UnixConfig { unix_socket: false },
};
