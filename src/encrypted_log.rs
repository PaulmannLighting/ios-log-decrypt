mod cha_cha_hex_stream;

use cha_cha_hex_stream::ChaChaHexStream;

/// An encrypted log file.
#[derive(Debug, Eq, PartialEq)]
pub struct EncryptedLog(String);

impl EncryptedLog {
    /// Create a new encrypted log file from whitespace-separated blocks of hex strings.
    #[must_use]
    pub const fn new(blocks: String) -> Self {
        Self(blocks)
    }

    /// Decrypt the ciphertext.
    #[must_use]
    pub fn decrypt(&self, key: &[u8]) -> ChaChaHexStream {
        ChaChaHexStream::new(&self.0, key)
    }
}
