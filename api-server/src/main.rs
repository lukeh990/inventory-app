/*
 * main.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

mod config;
mod log;

use std::process;

#[tokio::main]
async fn main() {
    let exit_code = run().await;
    process::exit(exit_code);
}

async fn run() -> i32 {
    let config = match config::read_config().await {
        Ok(x) => x,
        Err(e) => {
            log::failed_init_error(e);
            return 1;
        }
    };
    let _ = log::start_logging(config.logging_mode).await;
    0
}
