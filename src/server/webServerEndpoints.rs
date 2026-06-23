use crate::{security, server};
use actix_web::HttpResponse;
use actix_web::web;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Write;
use std::sync::atomic;
use tokio::time::Instant;

// Handling ping endpoint
pub async fn HandlePingEndpoint() -> HttpResponse {
    let START = Instant::now();
    let RESPONSE_TIME_MS = START.elapsed().as_millis();

    HttpResponse::Ok().json(json!({
        "message": "pong",
        "RESPONSE_TIME_MS": RESPONSE_TIME_MS
    }))
}

// Handling initialized status endpoint
pub async fn HandleInitializedStatusEndpoint() -> HttpResponse {
    if crate::isInitialized.load(atomic::Ordering::SeqCst) {
        println!(
            "{0}",
            "Server is initialized | HandleInitializedStatusEndpoint:  _".green()
        );
        return HttpResponse::Ok().finish();
    } else {
        println!(
            "{0}",
            "Server is not initialized | HandleInitializedStatusEndpoint:  _".red()
        );
        return HttpResponse::NoContent().finish();
    }
}

// Handling initialize server endpoint
#[derive(Deserialize)]
pub struct SCFAdminDetailsExtendedFLNR {
    #[serde(flatten)]
    admin: crate::SCFAdminDetails,
    pub keyBinHash: String,
}
pub async fn HandleInitializeServerEndpoint(
    req: web::Json<SCFAdminDetailsExtendedFLNR>,
) -> HttpResponse {
    // Getting req data
    let MAC_ADDRESS = &req.admin.macAddress;
    let NAME = &req.admin.name;
    let USERNAME = &req.admin.username;
    let PASSWORD = &req.admin.password;
    let KEY_BIN_HASH = &req.keyBinHash;

    // Verifying hash
    if let Ok(ACTUAL_KEY_BIN_HASH) = security::encryptionHandler::ConfigEncryptionKeyHash() {
        if KEY_BIN_HASH != &ACTUAL_KEY_BIN_HASH {
            println!(
                "{0}",
                "KEY_BIN_HASH compairison failed | HandleInitializeServerEndpoint: _".red()
            );
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid key bin hash"}));
        }
    }

    // Format checking
    if NAME.is_empty() {
        println!(
            "{0}",
            "NAME was empty form request | HandleInitializeServerEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Name cannot be empty"}));
    }
    if MAC_ADDRESS.is_empty() || !crate::MAC_ADDRESS_FORMAT.is_match(&MAC_ADDRESS) {
        println!(
            "{0}",
            "MAC_ADDRESS was empty form request | HandleInitializeServerEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Invalid admin mac address"}));
    }
    if (USERNAME.is_empty()) || (USERNAME.contains(' ')) {
        println!(
            "{0}",
            "USERNAME was empty form request | HandleInitializeServerEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized()
            .json(json!({"response": "Username cannot contain spaces"}));
    }
    if PASSWORD.is_empty() {
        println!(
            "{0}",
            "PASSWORD was empty form request | HandleInitializeServerEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Password cannot be empty"}));
    }

    // Initialize config file
    match server::InitializeConfigFile(NAME, MAC_ADDRESS, USERNAME, PASSWORD) {
        Ok(_) => {
            println!(
                "{0}",
                "Successfully initialized server | HandleInitializeServerEndpoint:  _".green()
            );
            return HttpResponse::Ok().finish();
        }
        Err(E) => {
            println!(
                "{0} {1:?}",
                "Initialize Server Endpoint Error | HandleInitializeServerEndpoint:  ".red(),
                E
            );
            return HttpResponse::InternalServerError().finish();
        }
    };
}

// Handling login verification endpoint
pub async fn HandleLoginVerificationEndpoint(
    req: web::Json<SCFAdminDetailsExtendedFLNR>,
) -> HttpResponse {
    // Getting req data
    let MAC_ADDRESS = &req.admin.macAddress;
    let USERNAME = &req.admin.username;
    let PASSWORD = &req.admin.password;
    let KEY_BIN_HASH = &req.keyBinHash;

    // Verifying hash
    if let Ok(ACTUAL_KEY_BIN_HASH) = security::encryptionHandler::ConfigEncryptionKeyHash() {
        if KEY_BIN_HASH != &ACTUAL_KEY_BIN_HASH {
            println!(
                "{0}",
                "KEY_BIN_HASH compairison failed | HandleLoginVerificationEndpoint: _".red()
            );
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid key bin hash"}));
        }
    }

    // Format checking
    if MAC_ADDRESS.is_empty() || !crate::MAC_ADDRESS_FORMAT.is_match(MAC_ADDRESS) {
        println!(
            "{0}",
            "MAC_ADDRESS was empty form request | HandleLoginVerificationEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Invalid admin mac address"}));
    }

    // Decrypting data
    let DECRYPTED_DATA: crate::ServerConfigFile =
        match security::encryptionHandler::DecryptConfigData() {
            Ok(DATA) => DATA,
            Err(E) => {
                println!(
                    "{0} {1:?}",
                    "Error decrypting config file (DECRYPTED_DATA) | HandleLoginVerificationEndpoint:  ".red(),
                    E
                );
                return HttpResponse::InternalServerError()
                    .json(json!({"response": "Internal Server Error"}));
            }
        };

    // Checking if admin mac is valid
    for i in 0..DECRYPTED_DATA.adminDetails.len() {
        if &DECRYPTED_DATA.adminDetails[i].macAddress != MAC_ADDRESS {
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid credentials"}));
        }

        // Checking if username is valid
        if &DECRYPTED_DATA.adminDetails[i].username != USERNAME {
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid credentials"}));
        }

        // Checking if password is valid
        if &DECRYPTED_DATA.adminDetails[i].password != PASSWORD {
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid credentials"}));
        }
    }

    // Return
    println!(
        "{0}",
        "Returning success | HandleLoginVerificationEndpoint:  _".green()
    );
    HttpResponse::Ok().finish()
}

// Handling Current Admin Details endpoint
#[derive(Deserialize)]
pub struct CurrentAdminDetailsRequest {
    pub macAddress: String,
    pub keyBinHash: String,
}

#[derive(Serialize)]
pub struct CurrentAdminDetails {
    pub name: String,
    pub macAddress: String,
    pub username: String,
}

// Handles the current admin details endpoint
pub async fn HandleCurrentAdminDetailsEndpoint(
    req: web::Json<CurrentAdminDetailsRequest>,
) -> HttpResponse {
    // Getting req data
    let MAC_ADDRESS = &req.macAddress;
    let KEY_BIN_HASH = &req.keyBinHash;

    // Verifying hash
    if let Ok(ACTUAL_KEY_BIN_HASH) = security::encryptionHandler::ConfigEncryptionKeyHash() {
        if KEY_BIN_HASH != &ACTUAL_KEY_BIN_HASH {
            println!(
                "{0}",
                "KEY_BIN_HASH compairison failed | HandleCurrentAdminDetailsEndpoint: _".red()
            );
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid key bin hash"}));
        }
    }

    // Decrypting data
    let DECRYPTED_DATA: crate::ServerConfigFile =
        match security::encryptionHandler::DecryptConfigData() {
            Ok(DATA) => DATA,
            Err(E) => {
                println!(
                    "{0} {1:?}",
                    "Error decrypting config file (DECRYPTED_DATA) | HandleCurrentAdminDetailsEndpoint:  ".red(),
                    E
                );
                return HttpResponse::InternalServerError()
                    .json(json!({"response": "Internal Server Error"}));
            }
        };

    // Checking if admin mac is valid
    let mut adminIndex = 0;
    let mut adminFound = false;

    for i in 0..DECRYPTED_DATA.adminDetails.len() {
        if &DECRYPTED_DATA.adminDetails[i].macAddress == MAC_ADDRESS {
            adminFound = true;
            adminIndex = i;
        }
    }

    if !adminFound {
        println!(
            "{0}",
            "Admin not found | HandleCurrentAdminDetailsEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Invalid credentials"}));
    }

    // Sending Admin details
    let ADMIN_DETAILS: CurrentAdminDetails = CurrentAdminDetails {
        name: DECRYPTED_DATA.adminDetails[adminIndex].name.clone(),
        macAddress: DECRYPTED_DATA.adminDetails[adminIndex].macAddress.clone(),
        username: DECRYPTED_DATA.adminDetails[adminIndex].username.clone(),
    };

    // Returning data
    println!(
        "{0}",
        "Returning admin details | HandleCurrentAdminDetailsEndpoint:  _".green()
    );
    return HttpResponse::Ok().json(json!({
        "response": "Success",
        "response": json!(ADMIN_DETAILS),
    }));
}

// Handle update password endpoint
#[derive(Deserialize)]
pub struct UpdateAdminPasswordRequest {
    pub macAddress: String,
    pub keyBinHash: String,
    pub currentPassword: String,
    pub newPassword: String,
    pub confirmPassword: String,
}
pub async fn HandleUpdateAdminPasswordEndpoint(
    req: web::Json<UpdateAdminPasswordRequest>,
) -> HttpResponse {
    // Getting req data
    let MAC_ADDRESS = &req.macAddress;
    let KEY_BIN_HASH = &req.keyBinHash;
    let CURRENT_PASSWORD = &req.currentPassword;
    let NEW_PASSWORD = &req.newPassword;
    let CONFIRM_PASSWORD = &req.confirmPassword;

    let mut adminFound = false;
    let mut adminIndex = 0;

    // Verifying hash
    if let Ok(ACTUAL_KEY_BIN_HASH) = security::encryptionHandler::ConfigEncryptionKeyHash() {
        if KEY_BIN_HASH != &ACTUAL_KEY_BIN_HASH {
            println!(
                "{0}",
                "KEY_BIN_HASH compairison failed | HandleUpdateAdminPasswordEndpoint: _".red()
            );
            return HttpResponse::Unauthorized().json(json!({"response": "Invalid key bin hash"}));
        }
    }

    // Format checking
    if MAC_ADDRESS.is_empty() || !crate::MAC_ADDRESS_FORMAT.is_match(MAC_ADDRESS) {
        println!(
            "{0}",
            "MAC_ADDRESS was empty form request | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Invalid admin mac address"}));
    }
    if CURRENT_PASSWORD.is_empty() {
        println!(
            "{0}",
            "CURRENT_PASSWORD was empty form request | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized()
            .json(json!({"response": "Current password cannot be empty"}));
    }
    if NEW_PASSWORD.is_empty() {
        println!(
            "{0}",
            "NEW_PASSWORD was empty form request | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized()
            .json(json!({"response": "New password cannot be empty"}));
    }
    if CONFIRM_PASSWORD.is_empty() {
        println!(
            "{0}",
            "CONFIRM_PASSWORD was empty form request | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized()
            .json(json!({"response": "Confirm password cannot be empty"}));
    }

    // Decrypting data
    let mut decryptedData: crate::ServerConfigFile =
        match security::encryptionHandler::DecryptConfigData() {
            Ok(DATA) => DATA,
            Err(E) => {
                println!(
                    "{0} {1:?}",
                    "Error decrypting config file (decryptedData) | HandleUpdateAdminPasswordEndpoint:  ".red(),
                    E
                );
                return HttpResponse::InternalServerError()
                    .json(json!({"response": "Internal Server Error"}));
            }
        };

    // Checking position of current admin
    for i in 0..decryptedData.adminDetails.len() {
        if &decryptedData.adminDetails[i].macAddress == MAC_ADDRESS {
            adminFound = true;
            adminIndex = i;
        }
    }

    if !adminFound {
        println!(
            "{0}",
            "Admin not found | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "MAC not found "}));
    }

    // Checking if current password is correct
    if CURRENT_PASSWORD.to_string() != decryptedData.adminDetails[adminIndex].password {
        println!(
            "{0}",
            "CURRENT_PASSWORD does not match | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized().json(json!({"response": "Wrong current password"}));
    }

    // Checking if new password is correct
    if NEW_PASSWORD != CONFIRM_PASSWORD {
        println!(
            "{0}",
            "NEW_PASSWORD and CONFIRM_PASSWORD do not match | HandleUpdateAdminPasswordEndpoint:  _".red()
        );
        return HttpResponse::Unauthorized()
            .json(json!({"response": "New and confirm password dosen't match"}));
    }

    decryptedData.adminDetails[adminIndex].password = NEW_PASSWORD.to_string();

    // Encrypting data
    let JSON_DATA = match serde_json::to_string_pretty(&decryptedData) {
        Ok(DATA) => DATA,
        Err(E) => {
            println!(
                "{0} {1:?}",
                "Error serializing config file (JSON_DATA) | HandleUpdateAdminPasswordEndpoint:  "
                    .red(),
                E
            );
            return HttpResponse::InternalServerError()
                .json(json!({"response": "Internal Server Error"}));
        }
    };

    let ENCRYPTED_DATA: Vec<u8> = match security::encryptionHandler::EncryptConfigData(
        JSON_DATA.as_bytes(),
    ) {
        Ok(DATA) => DATA,
        Err(E) => {
            println!(
                    "{0} {1:?}",
                    "Error encrypting config file (ENCRYPTED_DATA) | HandleUpdateAdminPasswordEndpoint:  ".red(),
                    E
                );
            return HttpResponse::InternalServerError()
                .json(json!({"response": "Internal Server Error"}));
        }
    };

    // Writing to config file
    let mut configFile = match server::CreateReturnConfigFile() {
        Ok(DATA) => DATA.file,
        Err(E) => {
            println!(
                "{0} {1:?}",
                "Error creating config file (configFile) | HandleUpdateAdminPasswordEndpoint:  "
                    .red(),
                E
            );
            return HttpResponse::InternalServerError()
                .json(json!({"response": "Internal Server Error"}));
        }
    };

    match configFile.write_all(ENCRYPTED_DATA.as_slice()) {
        Ok(_) => {
            println!(
                "{0}",
                "Successfully wrote to config file | HandleUpdateAdminPasswordEndpoint:  _".green()
            );
            return HttpResponse::Ok().finish();
        }
        Err(E) => {
            println!(
                "{0} {1:?}",
                "Error writing to config file (configFile.write_all) | HandleUpdateAdminPasswordEndpoint:  ".red(),
                E
            );
            return HttpResponse::InternalServerError()
                .json(json!({"response": "Internal Server Error"}));
        }
    };
}
