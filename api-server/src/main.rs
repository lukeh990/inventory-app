/*
 * main.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

mod config;

#[tokio::main]
async fn main() {
    let _ = config::read_config().await;
}
