mod block;

use block::Block;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit};
use hex::FromHex;
use std::iter::Map;
use std::str::SplitWhitespace;

type Blocks<'a> = Map<SplitWhitespace<'a>, fn(&str) -> Result<Block, anyhow::Error>>;

pub struct ChaChaHexStream<'a> {
    blocks: Blocks<'a>,
    cipher: ChaCha20Poly1305,
}

impl<'a> ChaChaHexStream<'a> {
    #[must_use]
    pub fn new(blocks: &'a str, key: &Key) -> Self {
        Self {
            blocks: blocks
                .split_whitespace()
                .map(|block| Block::from_hex(block)),
            cipher: ChaCha20Poly1305::new(key),
        }
    }
}

impl<'a> Iterator for ChaChaHexStream<'a> {
    type Item = anyhow::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.blocks.next().map(|block| block?.decrypt(&self.cipher))
    }
}
