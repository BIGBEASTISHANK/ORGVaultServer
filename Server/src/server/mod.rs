pub mod webServer;
use actix_web::{App, HttpServer, web};
use std::net::Ipv4Addr;

// Global Variables
pub const serverAddress: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const serverPort: u16 = 8020;

pub const globalProgramConfigFile: &str =
    "/home/ishank/Documents/ORGVault/Source Code/Server/TestingDIR/config.json"; // Production Location: "/etc/orgvault/config.json"

// Config file checking / creation
pub fn configFileGetter() -> std::fs::File {
    return match std::fs::File::open(globalProgramConfigFile) {
        // Found file
        Ok(cf) => {
            println!("Config File found... Proceding!");
            cf // Returning File
        }

        // Not found file
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                // Creating directory
                if let Some(parent) = std::path::Path::new(globalProgramConfigFile).parent() {
                    std::fs::create_dir_all(parent).expect("Error creating parent directories");
                    std::process::exit(1);
                }

                // Creating file
                let cf: std::fs::File = match std::fs::File::create(globalProgramConfigFile) {
                    Ok(file) => {
                        println!("Global config file created!");
                        file
                    }

                    Err(error) => {
                        println!("Error creating global config file: {:?}", error);
                        std::process::exit(1);
                    }
                };
                cf // Returning File
            }

            // Unexpected error occured
            other => {
                println!("Unexpected error opening config file: {:?}", other);
                std::process::exit(1);
            }
        },
    };
}

// APIEndpoint config function
fn apiEndpointConfig(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(webServer::webIndex));
}

// Server Main Function
pub async fn serverMain() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(apiEndpointConfig))
        .bind((serverAddress, serverPort))?
        .run()
        .await
}
