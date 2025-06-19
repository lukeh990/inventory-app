/*
 * config/v1.rs
 * Copyright (c) 2025 Luke Harding
 * This code is licensed under a MIT license.
 * See the file "LICENSE" in the root of this project.
 */

pub mod schema {
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    pub mod unix {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Clone)]
        pub struct Config {
            pub unix_socket: bool,
        }

        impl Default for Config {
            fn default() -> Self {
                Config { unix_socket: true }
            }
        }
    }

    #[derive(Deserialize, Serialize, Clone, Default)]
    pub enum LoggingMode {
        Syslog,
        Journald,
        #[default]
        Terminal,
        File(PathBuf),
        Discard,
    }

    #[derive(Deserialize, Serialize, Clone)]
    pub struct Config {
        pub version: u32,
        pub unix: Option<unix::Config>,
        pub logging_mode: Option<LoggingMode>,
    }

    impl Default for Config {
        fn default() -> Self {
            if cfg!(unix) {
                Config {
                    version: 1,
                    unix: Some(unix::Config::default()),
                    logging_mode: Some(LoggingMode::default()),
                }
            } else {
                Config {
                    version: 1,
                    unix: None,
                    logging_mode: Some(LoggingMode::default()),
                }
            }
        }
    }
}
