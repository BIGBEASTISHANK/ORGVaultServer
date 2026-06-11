use crate::server;
use colored::*;
use std::{path::Path, sync::atomic};

pub mod encryptionHandler;

// Security check function
pub fn VerifySecurityRequirements() -> Result<(), String> {
    // Config file exists
    println!("\t## Checking config file...");
    if Path::new(&*crate::GLOBAL_PROGRAM_CONFIG_FILE).exists() {
        println!("\t\t### Config file exists!");
        crate::REBUILD_FRONTEND.swap(false, atomic::Ordering::SeqCst);
    } else {
        println!(
            "{0} {1}",
            "\t\t### Config file does not exist!",
            "Generating....".green()
        );
        match server::InitializeConfigFile() {
            Ok(DATA) => {
                println!("\t\t### {}", DATA.fileFeedback);
                drop(DATA.file);
            }
            Err(e) => {
                return Err(format!(
                    "{0} {1}",
                    "Error getting config file: ".red(),
                    e.to_string().red()
                ));
            }
        };
    }

    // Checking encryption key file
    println!("\t## Checking encryption key file...");
    if Path::new(&*crate::GLOBAL_ENCRYPTION_KEY_FILE_LOCATION).exists() {
        println!("\t\t### Encryption key file exists!");
        crate::REBUILD_FRONTEND.swap(false, atomic::Ordering::SeqCst);
    } else {
        println!(
            "{0} {1}",
            "\t\t### Encryption key file does not exists!",
            "Generating....".green()
        );
        match encryptionHandler::GenerateConfigEncryptionKey() {
            Ok(_) => {
                println!("\t\t### Encryption key file generated!");
            }
            Err(e) => {
                return Err(format!(
                    "{0} {1}",
                    "Error generating encryption key file: ".red(),
                    e.red()
                ));
            }
        };
    }

    // Ok
    Ok(())
}
