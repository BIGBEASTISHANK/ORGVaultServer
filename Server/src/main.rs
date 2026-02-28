#![allow(nonstandard_style)]

use std;
use tokio;
use ORGVaultServer::server;

fn main() -> std::io::Result<()> {
    // Variable
    let tokioRT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();

    // Checking config file
    server::configFileGetter();

    // Starting feedback
    println!(
        "Starting ORGVault server on {0}:{1}",
        server::serverAddress,
        server::serverPort
    );

    // Starting Server
    return tokioRT.block_on(server::serverMain());
}
