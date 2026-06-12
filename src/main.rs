#![allow(nonstandard_style)]

use ORGVaultServer::{security, server};
use colored::*;
use std::{os::unix::process::CommandExt, process::Command};
use tokio;
use users;

// main function
fn main() -> std::io::Result<()> {
    // Make sure it run as root
    match RunAsRoot() {
        Ok(_) => {}
        Err(e) => {
            println!("{0} {1}", "Error: ".red(), e);
            return Ok(());
        }
    }

    // Security check
    println!("{0}", "# Starting security checks...".red());
    match security::VerifySecurityRequirements() {
        Ok(_) => {
            println!("{0}", "  ## Security check passed!\n".green());
        }
        Err(e) => {
            println!("{0} {1}", "Error: ".red(), e);
            std::process::exit(1);
        }
    }

    // Feedback
    let SERVER_FEEDBACK_IP = server::SERVER_ADDRESS.to_string();
    let SERVER_FEEDBACK_WEB_BACKEND_PORT = server::WEB_SERVER_BACKEND_PORT.to_string();
    let SERVER_FEEDBACK_CLIENT_COMMUNICATION_PORT = server::CLIENT_COMMUNICATION_PORT.to_string();

    println!(
        "{0}",
        "------------------------------------------------------------".bright_black()
    );
    println!(
        "{0} {1}:{2} {3} {4}:{5}",
        "▲ ORGVault Web back Server".cyan().bold(),
        SERVER_FEEDBACK_IP,
        SERVER_FEEDBACK_WEB_BACKEND_PORT,
        "| Client Communication:".bright_black(),
        SERVER_FEEDBACK_IP,
        SERVER_FEEDBACK_CLIENT_COMMUNICATION_PORT
    );
    println!(
        "{0}",
        "------------------------------------------------------------".bright_black()
    );

    // Start frontend
    let mut frontend = server::webServer::RunWebServerFrontend()?;

    // Starting web server
    let TOKIO_RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    let _ = TOKIO_RT.block_on(server::webServer::RunWebServerBackend());

    // Kill frontend
    let _ = frontend.kill();

    // Return
    Ok(())
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
        .arg(std::env::current_exe().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{} {:?}", "Error getting current exe:".red(), e),
            )
        })?)
        .args(std::env::args().skip(1))
        .exec();

    // Error
    return Err(SUDO_CONVERTION_ERROR.into());
}
