use chacha20poly1305::aead::Aead;
use chacha20poly1305::{aead, ChaCha20Poly1305};
use hex::{FromHex, FromHexError};

#[derive(Debug)]
pub struct EncryptedLine(Vec<u8>);

impl EncryptedLine {
    pub fn nonce(&self) -> &[u8] {
        &self.0[0..12]
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.0[12..]
    }

    pub fn decrypt(&self, cipher: &ChaCha20Poly1305) -> aead::Result<Vec<u8>> {
        cipher.decrypt(self.nonce().into(), self.ciphertext())
    }
}

impl FromHex for EncryptedLine {
    type Error = FromHexError;

    fn from_hex<T>(hex: T) -> Result<Self, Self::Error>
    where
        T: AsRef<[u8]>,
    {
        Vec::<u8>::from_hex(hex.as_ref()).map(Self)
    }
}
