#![allow(nonstandard_style)]

use ORGVaultServer::{security, server};
use colored::*;
use std;
use std::os::unix::process::CommandExt;
use std::process::Command;
use tokio;
use users;

// main function
fn main() -> std::io::Result<()> {
    // Make sure it run as root
    match RunAsRoot() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    }

    // Variables
    let SERVER_FEEDBACK_IP = server::SERVER_ADDRESS.to_string().red();
    let SERVER_FEEDBACK_PORT_WEB = server::WEB_SERVER_PORT.to_string().red();
    let SERVER_FEEDBACK_PORT_CLIENT = server::CLIENT_COMMUNICATION_PORT.to_string().red();
    let SERVER_FEEDBACK_COLON: ColoredString = ":".blue();

    // Variable
    let TOKIO_RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();

    // Security check
    println!("{}", "# Starting security checks...".red());
    match security::SecurityCheck() {
        Ok(_) => {
            println!("{}", "  ## Security check passed!\n".green());
        }
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    }

    // Starting feedback
    println!("-----------------------------------");
    println!(
        "\n{} {}{}{} {} {}{}{}",
        "Starting ORGVault web server on".blue(),
        SERVER_FEEDBACK_IP,
        SERVER_FEEDBACK_COLON,
        SERVER_FEEDBACK_PORT_WEB,
        "| Client Communication:".blue(),
        SERVER_FEEDBACK_IP,
        SERVER_FEEDBACK_COLON,
        SERVER_FEEDBACK_PORT_CLIENT
    );

    // Starting web server
    return TOKIO_RT.block_on(server::webServer::WebServerRunner());
}

// RunAsRoot function
fn RunAsRoot() -> std::io::Result<()> {
    // Check if we are root
    if users::get_current_uid() == 0 {
        return Ok(());
    }

    // Try to get root
    println!(
        "{}",
        "Root privileges required. Attempting to elevate via sudo...".yellow()
    );

    let SUDO_CONVERTION_ERROR: std::io::Error = Command::new("sudo")
        .arg(std::env::current_exe()?)
        .args(std::env::args().skip(1))
        .exec();

    // Error
    return Err(SUDO_CONVERTION_ERROR.into());
}
