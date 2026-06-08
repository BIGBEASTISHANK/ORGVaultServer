pub mod webServer;
use crate::configFileReturnValue;
use std::net::Ipv4Addr;

// Global Variables
pub const SERVER_ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const SERVER_PORT: u16 = 8020;

// Config file checking / creation
pub fn ConfigFileGetter() -> Result<configFileReturnValue, std::io::Error> {
    return match std::fs::File::open(crate::GLOBAL_PROGRAM_CONFIG_FILE) {
        // Found file
        Ok(cf) => Ok(configFileReturnValue {
            file: cf,
            status: true,
        }),

        // Not found file
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // Creating directory
                if let Some(parent) =
                    std::path::Path::new(crate::GLOBAL_PROGRAM_CONFIG_FILE).parent()
                {
                    std::fs::create_dir_all(parent).expect("Error creating parent directories");
                }

                // Creating file
                let cf: std::fs::File = std::fs::File::create(crate::GLOBAL_PROGRAM_CONFIG_FILE)
                    .expect("Error creating global config file");
                println!("Global config file created!");

                Ok(configFileReturnValue {
                    file: cf,
                    status: true,
                }) // Returning file and status
            }

            // Unexpected error occured
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected error occurred",
            )),
        },
    };
}
