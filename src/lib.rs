#![allow(nonstandard_style)]

use std::sync::atomic::{AtomicBool};

// Loading Libraries into crate
pub mod security;
pub mod server;

// Constant variables
pub const GLOBAL_PROGRAM_CONFIG_FILE: &str = if cfg!(debug_assertions) {
    "/run/media/ishank/Work/Projects/ORGVault/ORGVaultServer/GlobalConfigTesting/config.json"
} else {
    "/etc/orgvault/config.json"
};

pub const GLOBAL_ENCRYPTION_KEY_FILE_LOCATION: &str = if cfg!(debug_assertions) {
    "/run/media/ishank/Work/Projects/ORGVault/ORGVaultServer/GlobalConfigTesting/key.bin"
} else {
    "/etc/orgvault/key.bin"
};

pub const WEB_FRONTEND_DATA_FILE: &str = if cfg!(debug_assertions) {
    "/run/media/ishank/Work/Projects/ORGVault/ORGVaultServer/WebServerData"
} else {
    "/run/media/ishank/Work/Projects/ORGVault/ORGVaultServer/WebServerData"
};

// Mutable variables
pub static REBUILD_FRONTEND: AtomicBool = AtomicBool::new(true);

// Structs
pub struct CONFIG_FILE_RETURN_VALUE {
    pub file: std::fs::File,
    pub fileFeedback: String,
    pub status: bool,
}
