use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::RngCore;
use std::io::{Error, ErrorKind, Write};

// GenerateConfigEncryptionKey function
pub fn GenerateConfigEncryptionKey() -> Result<(), String> {
    let mut key = [0u8; 32];

    rand::thread_rng().fill_bytes(&mut key[..]);

    let mut file = std::fs::File::create(&*crate::GLOBAL_ENCRYPTION_KEY_FILE_LOCATION)
        .map_err(|e| e.to_string())?;

    match file.write_all(&key).map_err(|e| e.to_string()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
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

    let CIPHER = Aes256Gcm::new_from_slice(KEY).map_err(|e| Error::new(ErrorKind::Other, e))?;

    // 96 bytes nonce
    let mut nonceBytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonceBytes[..]);
    let NONCE = Nonce::from_slice(&nonceBytes[..]);

    let CIPHER_TEXT = CIPHER
        .encrypt(NONCE, DATA)
        .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?;

    let mut output = Vec::new();
    output.extend_from_slice(&nonceBytes);
    output.extend_from_slice(&CIPHER_TEXT);

    // Ok
    Ok(output)
}

// Decrypt data function
pub fn DecryptData(DATA: &[u8], KEY: &[u8]) -> Result<Vec<u8>, Error> {
    // Verifying data byte length
    if DATA.len() < 12 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Data length must be greater than 12 bytes",
        ));
    }

    let CIPHER = Aes256Gcm::new_from_slice(KEY).map_err(|e| Error::new(ErrorKind::Other, e))?;

    let (NONCE_BYTE, CIPHER_TEXT) = DATA.split_at(12);
    let NONCE = Nonce::from_slice(NONCE_BYTE);

    CIPHER
        .decrypt(NONCE, CIPHER_TEXT)
        .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))
}
