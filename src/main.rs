#![allow(nonstandard_style)]

use ORGVaultServer::server;
use std;
use tokio;

fn main() -> std::io::Result<()> {
    // Variable
    let tokioRT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();

    // Checking config file
    let configFile: std::fs::File = match server::ConfigFileGetter() {
        Ok(fileVec) => {
            println!("Config File found... Proceeding!");
            fileVec.file
        }
        Err(e) => {
            println!("Error getting config file: {:?}", e);
            std::process::exit(1);
        }
    };

    // Starting feedback
    println!(
        "Starting ORGVault server on {0}:{1}",
        server::SERVER_ADDRESS,
        server::SERVER_PORT
    );

    // Starting web server
    return tokioRT.block_on(server::webServer::WebServerRunner());
}
