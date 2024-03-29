use anyhow::anyhow;
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305};
use hex::FromHex;

const NONCE_SIZE: usize = 12;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
    nonce: [u8; NONCE_SIZE],
    ciphertext: Box<[u8]>,
}

impl Block {
    pub fn decrypt(&self, cipher: &ChaCha20Poly1305) -> anyhow::Result<Vec<u8>> {
        Ok(cipher.decrypt(self.nonce.as_slice().into(), self.ciphertext.as_ref())?)
    }
}

impl FromHex for Block {
    type Error = anyhow::Error;

    fn from_hex<T>(hex: T) -> Result<Self, Self::Error>
    where
        T: AsRef<[u8]>,
    {
        Self::try_from(Vec::<u8>::from_hex(hex)?.as_slice())
    }
}

impl TryFrom<&[u8]> for Block {
    type Error = anyhow::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            nonce: bytes
                .get(0..NONCE_SIZE)
                .ok_or_else(|| anyhow!("invalid nonce"))?
                .try_into()?,
            ciphertext: bytes
                .get(NONCE_SIZE..)
                .ok_or_else(|| anyhow!("invalid ciphertext"))?
                .into(),
        })
    }
}
