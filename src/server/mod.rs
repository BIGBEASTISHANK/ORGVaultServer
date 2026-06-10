pub mod webServer;
use std::net::Ipv4Addr;

// Server addr/port
pub const SERVER_ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const WEB_SERVER_PORT: u16 = 8020;
pub const CLIENT_COMMUNICATION_PORT: u16 = 8040;

// Config file checking / creation
pub fn ConfigFileGetter() -> Result<crate::CONFIG_FILE_RETURN_VALUE, std::io::Error> {
    return match std::fs::File::open(crate::GLOBAL_PROGRAM_CONFIG_FILE) {
        // Found file
        Ok(CF) => Ok(crate::CONFIG_FILE_RETURN_VALUE {
            file: CF,
            fileFeedback: "Global config file was found!".to_string(),
            status: true,
        }),

        // Not found file
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // Creating directory
                if let Some(PARENT) =
                    std::path::Path::new(crate::GLOBAL_PROGRAM_CONFIG_FILE).parent()
                {
                    std::fs::create_dir_all(PARENT).expect("Error creating parent directories");
                }

                // Creating file
                let CF: std::fs::File = std::fs::File::create(crate::GLOBAL_PROGRAM_CONFIG_FILE)
                    .expect("Error creating global config file");

                Ok(crate::CONFIG_FILE_RETURN_VALUE {
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
