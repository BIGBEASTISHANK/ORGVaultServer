#![allow(nonstandard_style)]

use regex::Regex;
use serde::{Deserialize, Serialize};
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
        "/usr/share/orgvault/config.json".to_string()
    }
});

pub static CONFIG_ENCRYPTION_KEY_FILE_LOCATION: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/GlobalConfigTesting/key.bin", *CURRENT_DIR)
    } else {
        "/etc/orgvault/key.bin".to_string()
    }
});

pub static COMMON_ENCRYPTION_KEY_FILE_LOCATION: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/GlobalConfigTesting/commonEncKey.bin", *CURRENT_DIR)
    } else {
        "/usr/share/orgvault/commonEncKey.bin".to_string()
    }
});

pub static WEB_FRONTEND_DATA_FILE: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/WebServerData", *CURRENT_DIR)
    } else {
        format!("{0}/WebServerData", *CURRENT_DIR)
    }
});

pub static MAC_ADDRESS_FORMAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$").unwrap());

pub static GPC_PLAIN_FILE_LOCATION: LazyLock<String> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        format!("{0}/GlobalConfigTesting/plainConfig.json", *CURRENT_DIR)
    } else {
        "".to_string()
    }
});

// Mutable variables
pub static rebuildFrontendStatus: AtomicBool = AtomicBool::new(true);
pub static isInitialized: AtomicBool = AtomicBool::new(false);

// Structs
pub struct ConfigFileReturnValue {
    pub file: std::fs::File,
    pub fileFeedback: String,
    pub status: bool,
}

// Server config file data structure
#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfigFile {
    pub serverDetails: SCFServerDetails,
    pub adminDetails: Vec<SCFAdminDetails>,
    pub managers: Vec<SCFManagers>,
    pub folders: Vec<SCFFolders>,
    pub employees: Vec<SCFEmployee>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SCFServerDetails {
    pub commonEncryptionKeyLoc: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SCFAdminDetails {
    pub name: String,
    pub macAddress: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SCFManagers {
    pub name: String,
    pub macAddress: String,
    pub publicKeyLoc: String,
    pub departments: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SCFFolders {
    pub folderName: String,
    pub department: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SCFEmployee {
    pub name: String,
    pub macAddress: String,
    pub publicKeyLoc: String,
    pub department: String,
}
