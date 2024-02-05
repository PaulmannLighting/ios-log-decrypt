mod encrypted_log;

pub use encrypted_log::EncryptedLog;

/// Decrypt iOS logs
///
/// # Errors
/// Returns an [`anyhow::Error`] if the log file could not be decrypted.
pub fn decrypt(ciphertext: &str, key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let encrypted = EncryptedLog::new(ciphertext.into());
    let blocks: Vec<anyhow::Result<Vec<u8>>> = encrypted.decrypt(key.into()).collect();
    let mut bytes = Vec::with_capacity(
        blocks
            .iter()
            .map(|result| result.as_ref().map(Vec::len).unwrap_or(0))
            .sum(),
    );

    for block in blocks {
        bytes.extend(block?);
    }

    Ok(bytes)
}
