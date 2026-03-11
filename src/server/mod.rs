pub mod webServer;
use std::net::Ipv4Addr;

// Global Variables
pub const SERVER_ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 8020;

pub const GLOBAL_PROGRAM_CONFIG_FILE: &str =
    "/home/ishank/Documents/ORGVault/ORGVaultServer/GlobalConfigTesting/config.json"; // Production Location: "/etc/orgvault/config.json"

// Config file checking / creation
pub fn ConfigFileGetter() -> std::fs::File {
    return match std::fs::File::open(GLOBAL_PROGRAM_CONFIG_FILE) {
        // Found file
        Ok(cf) => {
            println!("Config File found... Proceding!");
            cf // Returning File
        }

        // Not found file
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // Creating directory
                if let Some(parent) = std::path::Path::new(GLOBAL_PROGRAM_CONFIG_FILE).parent() {
                    std::fs::create_dir_all(parent).expect("Error creating parent directories");
                }

                // Creating file
                let cf: std::fs::File = std::fs::File::create(GLOBAL_PROGRAM_CONFIG_FILE)
                    .expect("Error creating global config file");
                println!("Global config file created!");

                cf // Returning File
            }

            // Unexpected error occured
            _ => {
                println!("Unexpected error opening config file: {:?}", e);
                std::process::exit(1);
            }
        },
    };
}
