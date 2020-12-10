use crate::block::Block;
use crate::hashable::Hashable;
use crate::lib::now;
use crate::transaction::{Address, Transaction, ROOT_ADDR};

const INITIAL_REWARD: f32 = 100.0;

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>,
    transactions: Vec<Transaction>,
    difficulty: u64,
    miner_addr: Address,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: Address, difficulty: u64) -> Self {
        let mut chain = Chain {
            blocks: Vec::new(),
            transactions: Vec::new(),
            difficulty,
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
            None => vec![0; 32],
        };
        let mut block = Block::new(now(), prev_block_hash, self.difficulty, transactions);
        Chain::proof_of_work(&mut block, self.difficulty as usize);
        self.blocks.push(block);
    }

    pub fn proof_of_work(block: &mut Block, difficulty: usize) {
        'main: loop {
            let hash = block.hash();
            for i in hash[..difficulty].into_iter() {
                if *i != 0 {
                    block.nonce += 1;
                    continue 'main;
                }
            }
            block.curr_block_hash = hash;
            return;
        }
    }
}
