pub mod webServer;
use std::{io::Write, net::Ipv4Addr};

// Server addr/port
pub const SERVER_ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const CLIENT_COMMUNICATION_PORT: u16 = 8040;
pub const WEB_SERVER_BACKEND_PORT: u16 = 3100;
pub const WEB_SERVER_FRONTEND_PORT: u16 = 3000;

// Config file checking / creation
pub fn CreateReturnConfigFile() -> Result<crate::ConfigFileReturnValue, std::io::Error> {
    return match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&*crate::GLOBAL_PROGRAM_CONFIG_FILE)
    {
        // Found file
        Ok(CF) => Ok(crate::ConfigFileReturnValue {
            file: CF,
            fileFeedback: "Global config file was found!".to_string(),
            status: true,
        }),

        // Not found file
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // Creating directory
                if let Some(PARENT) =
                    std::path::Path::new(&*crate::GLOBAL_PROGRAM_CONFIG_FILE).parent()
                {
                    std::fs::create_dir_all(PARENT).expect("Error creating parent directories");
                }

                // Creating file
                let CF: std::fs::File = std::fs::File::create(&*crate::GLOBAL_PROGRAM_CONFIG_FILE)
                    .expect("Error creating global config file");

                Ok(crate::ConfigFileReturnValue {
                    file: CF,
                    fileFeedback: "Global config file was created!".to_string(),
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

// Initialize config file
pub fn InitializeConfigFile(
    MAC_ADDRESS: String,
    USERNAME: String,
    PASSWORD: String,
) -> Result<(), std::io::Error> {
    // Creating config file
    let mut configFile: std::fs::File = CreateReturnConfigFile()?.file;

    // Creating config file data
    let CONFIG_FILE_DATA: crate::ServerConfigFile = crate::ServerConfigFile {
        adminDetails: vec![crate::SCFAdminDetails {
            macAddress: MAC_ADDRESS,
            username: USERNAME,
            password: PASSWORD,
        }],
    };

    // Converting to JSON
    let JSON = serde_json::to_string_pretty(&CONFIG_FILE_DATA)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Writing to file
    configFile.write_all(JSON.as_bytes())?;

    // Returning
    Ok(())
}
