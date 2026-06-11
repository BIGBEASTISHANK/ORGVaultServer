use rand::RngCore;
use std::io::Write;

// GenerateConfigEncryptionKey function
pub fn GenerateConfigEncryptionKey() -> Result<(), String> {
    let mut key = [0u8; 32];

    rand::thread_rng().fill_bytes(&mut key[..]);

    let mut file = std::fs::File::create(&*crate::GLOBAL_ENCRYPTION_KEY_FILE_LOCATION)
        .map_err(|e| e.to_string())?;

    file.write_all(&key).map_err(|e| e.to_string())?;

    Ok(())
}
