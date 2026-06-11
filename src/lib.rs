#![allow(nonstandard_style)]

use std::{
    env,
    sync::{LazyLock, atomic::AtomicBool},
};

// Loading Libraries into crate
pub mod security;
pub mod server;

// Private variables
static CURRENT_DIR: LazyLock<String> =
    LazyLock::new(|| env::current_dir().unwrap().display().to_string());

// Public Constant variables
pub static GLOBAL_PROGRAM_CONFIG_FILE: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/GlobalConfigTesting/config.json", *CURRENT_DIR)
    } else {
        "/etc/orgvault/config.json".to_string()
    }
});

pub static GLOBAL_ENCRYPTION_KEY_FILE_LOCATION: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/GlobalConfigTesting/key.bin", *CURRENT_DIR)
    } else {
        "/etc/orgvault/key.bin".to_string()
    }
});

pub static WEB_FRONTEND_DATA_FILE: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/WebServerData", *CURRENT_DIR)
    } else {
        format!("{0}/WebServerData", *CURRENT_DIR)
    }
});

// Mutable variables
pub static REBUILD_FRONTEND: AtomicBool = AtomicBool::new(true);

// Structs
pub struct ConfigFileReturnValue {
    pub file: std::fs::File,
    pub fileFeedback: String,
    pub status: bool,
}
