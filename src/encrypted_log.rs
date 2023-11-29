mod cha_cha_hex_stream;

use cha_cha_hex_stream::ChaChaHexStream;

/// An encrypted log file.
#[derive(Debug, Eq, PartialEq)]
pub struct EncryptedLog(String);

impl EncryptedLog {
    /// Create a new encrypted log file from a header and ciphertext.
    #[must_use]
    pub const fn new(lines: String) -> Self {
        Self(lines)
    }

    /// Decrypt the ciphertext.
    #[must_use]
    pub fn decrypt(&self, key: &[u8]) -> ChaChaHexStream {
        ChaChaHexStream::new(self.0.as_str(), key)
    }
}
