#![allow(nonstandard_style)]

use std;
use tokio;
use ORGVaultServer::server;

fn main() -> std::io::Result<()> {
    // Variable
    let tokioRT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();

    // Checking config file
    server::ConfigFileGetter();

    // Starting feedback
    println!(
        "Starting ORGVault server on {0}:{1}",
        server::SERVER_ADDRESS,
        server::SERVER_PORT
    );

    // Starting web server
    return tokioRT.block_on(server::webServer::WebServerRunner());
}
