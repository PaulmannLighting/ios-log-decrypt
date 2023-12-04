use chacha20poly1305::aead::Aead;
use chacha20poly1305::{aead, ChaCha20Poly1305};
use hex::{FromHex, FromHexError};

const NONCE_SIZE: usize = 12;

#[derive(Debug)]
pub struct Block(Box<[u8]>);

impl Block {
    pub fn nonce(&self) -> &[u8] {
        &self.0[0..NONCE_SIZE]
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.0[NONCE_SIZE..]
    }

    pub fn decrypt(&self, cipher: &ChaCha20Poly1305) -> aead::Result<Vec<u8>> {
        cipher.decrypt(self.nonce().into(), self.ciphertext())
    }
}

impl FromHex for Block {
    type Error = FromHexError;

    fn from_hex<T>(hex: T) -> Result<Self, Self::Error>
    where
        T: AsRef<[u8]>,
    {
        Vec::<u8>::from_hex(hex)
            .map(Vec::into_boxed_slice)
            .map(Self)
    }
}
