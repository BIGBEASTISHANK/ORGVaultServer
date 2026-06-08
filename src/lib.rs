#![allow(nonstandard_style)]

// Loading Libraries into crate
pub mod security;
pub mod server;

pub const GLOBAL_PROGRAM_CONFIG_FILE: &str =
    "/home/ishank/Documents/ORGVault/ORGVaultServer/GlobalConfigTesting/config.json"; // Production Location: "/etc/orgvault/config.json"

#[derive(Debug)]
pub struct configFileReturnValue {
    pub file: std::fs::File,
    pub status: bool,
}
