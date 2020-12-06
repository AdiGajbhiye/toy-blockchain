use crate::hashable::{Hashable,Bytes};

pub type Address = &'static str;

pub const ROOT_ADDR: Address = "ROOT";

#[derive(Debug)]
pub struct Transaction {
    sender: Address,
    reciever: Address,
    amount: f32,
}

impl Transaction {
    pub fn new(sender: Address, reciever: Address, amount: f32) -> Self {
        Transaction {
            sender,
            reciever,
            amount,
        }
    }
}

impl Hashable for Address {
    fn bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Bytes {
        let mut result = Vec::new();
        result.extend(self.sender.bytes());
        result.extend(self.reciever.bytes());
        result.extend(&self.amount.to_bits().to_be_bytes());
        result
    }
}