use crate::hashable::{Bytes, Hashable};
use crate::lib::{u128_to_bytes, u64_to_bytes};
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub timestamp: u128,
    pub curr_block_hash: Bytes,
    pub prev_block_hash: Bytes,
    pub nonce: u64,
    pub payload: Vec<Transaction>,
}

impl Block {
    pub fn new(
        timestamp: u128,
        prev_block_hash: Bytes,
        nonce: u64,
        payload: Vec<Transaction>,
    ) -> Self {
        Block {
            timestamp,
            curr_block_hash: vec![0; 64],
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Bytes {
        let mut result = Vec::new();
        result.extend(u64_to_bytes(self.nonce).iter());
        result.extend(u128_to_bytes(self.timestamp).iter());
        result.extend(self.prev_block_hash.clone());
        result.extend(self.payload.iter().flat_map(|x| x.hash()));
        result
    }
}
