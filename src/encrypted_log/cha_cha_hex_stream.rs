mod block;

use block::Block;
use chacha20poly1305::{aead, ChaCha20Poly1305, KeyInit};
use hex::FromHex;
use std::iter::FilterMap;
use std::str::SplitWhitespace;

pub struct ChaChaHexStream<'a> {
    blocks: FilterMap<SplitWhitespace<'a>, fn(&str) -> Option<Block>>,
    cipher: ChaCha20Poly1305,
}

impl<'a> ChaChaHexStream<'a> {
    #[must_use]
    pub fn new(blocks: &'a str, key: &[u8]) -> Self {
        Self {
            blocks: blocks
                .split_whitespace()
                .filter_map(|block| Block::from_hex(block).ok()),
            cipher: ChaCha20Poly1305::new(key.into()),
        }
    }
}

impl<'a> Iterator for ChaChaHexStream<'a> {
    type Item = aead::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.next().map(|line| line.decrypt(&self.cipher))
    }
}
