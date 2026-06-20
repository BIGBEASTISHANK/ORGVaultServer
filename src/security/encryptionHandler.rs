use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use colored::*;
use rand::RngCore;
use std::{
    fs,
    io::{Error, ErrorKind, Read, Write},
    process::Command,
};

// GenerateConfigEncryptionKey function
#[derive(PartialEq, Debug)]
pub enum EncryptionKeyType {
    ConfigKey,
    CommonKey,
}
pub fn GenerateConfigEncryptionKey(EKT: EncryptionKeyType) -> Result<(), Error> {
    let mut key = [0u8; 32];

    rand::thread_rng().fill_bytes(&mut key[..]);

    let KEY_FILE_LOCATION = if EKT == EncryptionKeyType::ConfigKey {
        &*crate::GLOBAL_ENCRYPTION_KEY_FILE_LOCATION
    } else if EKT == EncryptionKeyType::CommonKey {
        &*crate::GLOBAL_COMMON_ENCRYPTION_KEY_FILE_LOCATION
    } else {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}: {2:?}",
                "Generate Config Encryption Key Error (file creation) | GenerateConfigEncryptionKey | Key Type Code:".red(),
                EKT,
                "Invalid encryption key type"
            ),
        ));
    };

    let mut file =
        std::fs::File::create(KEY_FILE_LOCATION).map_err(|E| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "{0} {1:?}",
                    "Generate Config Encryption Key Error (file creation) | GenerateConfigEncryptionKey:  ".red(),
                    E
                ),
            )
        })?;

    file.write_all(&key).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Generate Config Encryption Key Error (file writting) | GenerateConfigEncryptionKey:  ".red(),
                E
            ),
        )
    })?;

    Ok(())
}

// ConfigEncryptionKeyHash function
pub fn ConfigEncryptionKeyHash() -> Result<String, ()> {
    // Running command and returning output
    let COMMAND_OUTPUT = Command::new("sha256sum")
        .arg(&*crate::GLOBAL_ENCRYPTION_KEY_FILE_LOCATION)
        .output()
        .map_err(|E| {
            println!(
                "{0} {1:?}",
                "Config Encryption Key Hash Error | ConfigEncryptionKeyHash:  ".red(),
                E
            );
            return ();
        })
        .unwrap()
        .stdout;
    let COMMAND_OUTPUT = String::from_utf8(COMMAND_OUTPUT).unwrap();
    let HASH_OUTPUT: String = match COMMAND_OUTPUT.split_whitespace().next() {
        Some(HASH) => String::from(HASH),
        _ => "".to_string(),
    };

    // Error
    if HASH_OUTPUT.is_empty() {
        return Err(());
    }

    // Return
    Ok(HASH_OUTPUT)
}

// Encrypt data function
pub fn EncryptData(DATA: &[u8], KEY: &[u8]) -> Result<Vec<u8>, Error> {
    // Verifying key length
    if KEY.len() != 32 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Key length must be 32 bytes",
        ));
    }

    let CIPHER = Aes256Gcm::new_from_slice(KEY).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Encrypt Data Error (creating cipher) | EncryptData:  ".red(),
                E
            ),
        )
    })?;

    // 96 bytes nonce
    let mut nonceBytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonceBytes[..]);
    let NONCE = Nonce::from_slice(&nonceBytes[..]);

    let CIPHER_TEXT = CIPHER.encrypt(NONCE, DATA).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Encrypt Data Error (encrypting) | EncryptData:  ".red(),
                E
            ),
        )
    })?;

    let mut output = Vec::new();
    output.extend_from_slice(&nonceBytes);
    output.extend_from_slice(&CIPHER_TEXT);

    // Ok
    Ok(output)
}

// Decrypt data function
pub fn DecryptConfigData() -> Result<crate::ServerConfigFile, Error> {
    let mut configFile: fs::File = match fs::File::open(&*crate::GLOBAL_PROGRAM_CONFIG_FILE) {
        Ok(FILE) => FILE,
        Err(E) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{0} {1:?}",
                    "Unable to get config file Error (configFile) | DecryptData:  ".red(),
                    E
                ),
            ));
        }
    };
    let mut keyFile: fs::File = match fs::File::open(&*crate::GLOBAL_ENCRYPTION_KEY_FILE_LOCATION) {
        Ok(FILE) => FILE,
        Err(E) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{0} {1:?}",
                    "Unable to get key file Error (keyFile) | DecryptData:  ".red(),
                    E
                ),
            ));
        }
    };

    let mut configFileDataBuffer = Vec::new();
    configFile
        .read_to_end(&mut configFileDataBuffer)
        .map_err(|E| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "{0} {1:?}",
                    "Unable to read config file | DecryptData:".red(),
                    E
                ),
            )
        })?;

    let mut keyFileDataBuffer = Vec::new();
    keyFile.read_to_end(&mut keyFileDataBuffer).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Unable to read key file | DecryptData:".red(),
                E
            ),
        )
    })?;

    // AES-256 requires a 32-byte key
    if keyFileDataBuffer.len() != 32 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "{0} {1} {2}",
                "Invalid AES-256 key length. Expected 32 bytes, found".red(),
                keyFileDataBuffer.len(),
                "bytes.".red(),
            ),
        ));
    }

    // Verifying configFileDataBuffer byte length
    if configFileDataBuffer.len() < 12 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("{0}", "Data length must be greater than 12 bytes | DecryptData".red()),
        ));
    }

    let CIPHER = Aes256Gcm::new_from_slice(&keyFileDataBuffer).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Decrypt Data Error (creating cipher) | DecryptData:  ".red(),
                E
            ),
        )
    })?;

    let (NONCE_BYTE, CIPHER_TEXT) = configFileDataBuffer.split_at(12);
    let NONCE = Nonce::from_slice(NONCE_BYTE);

    let DECRYPTED_DATA = CIPHER.decrypt(NONCE, CIPHER_TEXT).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Error decrypting data | DecryptData:  ".red(),
                E
            ),
        )
    })?;

    let CONFIG: crate::ServerConfigFile = serde_json::from_slice(&DECRYPTED_DATA).map_err(|E| {
        Error::new(
            ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Error deserializing decrypted config | DecryptData:".red(),
                E
            ),
        )
    })?;

    Ok(CONFIG)
}
