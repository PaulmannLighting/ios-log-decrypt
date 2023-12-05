use anyhow::anyhow;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::ChaCha20Poly1305;
use hex::{FromHex, FromHexError};

const NONCE_SIZE: usize = 12;

#[derive(Debug)]
pub struct Block(Box<[u8]>);

impl Block {
    pub fn nonce(&self) -> anyhow::Result<&[u8]> {
        self.0
            .get(0..NONCE_SIZE)
            .ok_or_else(|| anyhow!("invalid nonce"))
    }

    pub fn ciphertext(&self) -> anyhow::Result<&[u8]> {
        self.0
            .get(NONCE_SIZE..)
            .ok_or_else(|| anyhow!("invalid ciphertext"))
    }

    pub fn decrypt(&self, cipher: &ChaCha20Poly1305) -> anyhow::Result<Vec<u8>> {
        Ok(cipher.decrypt(self.nonce()?.into(), self.ciphertext()?)?)
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
