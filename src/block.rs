use crate::transaction::Transaction;
use crate::hashable::{Hashable,Bytes};

#[derive(Debug)]
pub struct Block {
    pub index: usize,
    pub timestamp: u128,
    pub curr_block_hash: Bytes,
    pub prev_block_hash: Bytes,
    pub nonce: u32,
    pub payload: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: usize,
        timestamp: u128,
        prev_block_hash: Bytes,
        nonce: u32,
        payload: Vec<Transaction>,
    ) -> Self {
        Block {
            index,
            timestamp,
            curr_block_hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Bytes {
        let mut result = Vec::new();
        result.extend(self.prev_block_hash.clone());
        result.extend(self.payload.iter().flat_map(|x| x.hash()));
        result
    }
}