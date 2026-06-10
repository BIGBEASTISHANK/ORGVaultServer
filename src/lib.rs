#![allow(nonstandard_style)]

// Loading Libraries into crate
pub mod security;
pub mod server;

pub const GLOBAL_PROGRAM_CONFIG_FILE: &str =
    "/run/media/ishank/Work/Projects/ORGVault/ORGVaultServer/GlobalConfigTesting/config.json"; // Production Location: "/etc/orgvault/config.json"
pub const GLOBAL_ENCRYPTION_KEY_FILE_LOCATION: &str =
    "/run/media/ishank/Work/Projects/ORGVault/ORGVaultServer/GlobalConfigTesting/key.bin"; // Production Location: "/etc/orgvault/key.bin"

pub struct CONFIG_FILE_RETURN_VALUE {
    pub file: std::fs::File,
    pub fileFeedback: String,
    pub status: bool,
}
