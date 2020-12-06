use crate::block::Block;
use crate::lib::now;
use crate::transaction::{Address, Transaction, ROOT_ADDR};

const INITIAL_REWARD: f32 = 100.0;

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>,
    transactions: Vec<Transaction>,
    nonce: u32,
    miner_addr: Address,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: Address, nonce: u32) -> Self {
        let mut chain = Chain {
            blocks: Vec::new(),
            transactions: Vec::new(),
            nonce,
            miner_addr,
            reward: INITIAL_REWARD,
        };
        chain.generate_block();
        chain
    }

    pub fn add_transaction(&mut self, sender: Address, reciever: Address, amount: f32) {
        self.transactions
            .push(Transaction::new(sender, reciever, amount));
    }

    pub fn generate_block(&mut self) {
        let mut transactions = Vec::new();
        transactions.push(Transaction::new(ROOT_ADDR, self.miner_addr, self.reward));
        transactions.append(&mut self.transactions);
        let prev_block_hash = match self.blocks.last() {
            Some(b) => b.curr_block_hash.clone(),
            None => vec![48; 64],
        };
        self.blocks.push(Block::new(0, now(), prev_block_hash, self.nonce, transactions))
    }
}
