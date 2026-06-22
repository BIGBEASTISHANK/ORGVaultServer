pub mod webServer;
pub mod webServerEndpoints;
use std::{
    fs,
    io::{Error, ErrorKind, Write},
    net::Ipv4Addr,
    sync::atomic::Ordering,
};

use crate::security::encryptionHandler::{
    EncryptConfigData, EncryptionKeyType, GenerateConfigEncryptionKey,
};
use colored::*;

// Server addr/port
pub const SERVER_ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const CLIENT_COMMUNICATION_PORT: u16 = 8040;
pub const WEB_SERVER_BACKEND_PORT: u16 = 3100;
pub const WEB_SERVER_FRONTEND_PORT: u16 = 3000;

// Config file checking / creation
pub fn CreateReturnConfigFile() -> Result<crate::ConfigFileReturnValue, Error> {
    return match fs::OpenOptions::new()
        .read(true)
        .truncate(true)
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
            ErrorKind::NotFound => {
                // Creating directory
                if let Some(PARENT) =
                    std::path::Path::new(&*crate::GLOBAL_PROGRAM_CONFIG_FILE).parent()
                {
                    fs::create_dir_all(PARENT).expect("Error creating parent directories");
                }

                // Creating file
                let CF: fs::File = fs::File::create(&*crate::GLOBAL_PROGRAM_CONFIG_FILE)
                    .expect("Error creating global config file");

                Ok(crate::ConfigFileReturnValue {
                    file: CF,
                    fileFeedback: "Global config file was created!".to_string(),
                    status: true,
                }) // Returning file and status
            }

            // Unexpected error occured
            _ => Err(Error::new(
                ErrorKind::Other,
                "Unexpected error occurred",
            )),
        },
    };
}

// Initialize config file
pub fn InitializeConfigFile(
    NAME: &String,
    MAC_ADDRESS: &String,
    USERNAME: &String,
    PASSWORD: &String,
) -> Result<(), Error> {
    // Creating config file
    let mut configFile: fs::File = CreateReturnConfigFile()?.file;

    // Createing common enc key
    if let Err(E) = GenerateConfigEncryptionKey(EncryptionKeyType::CommonKey) {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Error generating common enc key | VerifySecurityRequirements:  ".red(),
                E
            ),
        ));
    };

    // Creating config file data
    let CONFIG_FILE_DATA: crate::ServerConfigFile = crate::ServerConfigFile {
        serverDetails: crate::SCFServerDetails {
            commonEncryptionKeyLoc: crate::GLOBAL_COMMON_ENCRYPTION_KEY_FILE_LOCATION.to_string(),
        },
        adminDetails: vec![crate::SCFAdminDetails {
            name: NAME.to_string(),
            macAddress: MAC_ADDRESS.to_string(),
            username: USERNAME.to_string(),
            password: PASSWORD.to_string(),
        }],
        managers: Vec::new(),
        folders: Vec::new(),
        employees: Vec::new(),
    };

    // Converting to JSON
    let JSON_DATA = serde_json::to_string_pretty(&CONFIG_FILE_DATA)
        .map_err(|E| Error::new(ErrorKind::Other, E))?;

    let ENCRYPTED_DATA = EncryptConfigData(JSON_DATA.as_bytes());

    // Writing to file
    configFile.write_all(ENCRYPTED_DATA?.as_slice())?;

    // Setting initialized state
    crate::isInitialized.swap(true, Ordering::SeqCst);

    // Returning
    Ok(())
}
