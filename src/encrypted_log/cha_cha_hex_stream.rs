mod encrypted_line;

use chacha20poly1305::{aead, ChaCha20Poly1305, KeyInit};
use encrypted_line::EncryptedLine;
use hex::FromHex;
use std::iter::FilterMap;
use std::str::SplitWhitespace;

pub struct ChaChaHexStream<'a> {
    lines: FilterMap<SplitWhitespace<'a>, fn(&str) -> Option<EncryptedLine>>,
    cipher: ChaCha20Poly1305,
}

impl<'a> ChaChaHexStream<'a> {
    #[must_use]
    pub fn new(lines: &'a str, key: &[u8]) -> Self {
        Self {
            lines: lines
                .split_whitespace()
                .filter_map(|line| EncryptedLine::from_hex(line).ok()),
            cipher: ChaCha20Poly1305::new(key.into()),
        }
    }
}

impl<'a> Iterator for ChaChaHexStream<'a> {
    type Item = aead::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| line.decrypt(&self.cipher))
    }
}
