mod block;
mod chain;
mod hashable;
mod lib;
mod transaction;

use chain::Chain;
use transaction::ROOT_ADDR;

fn main() {
    let mut chain = Chain::new(ROOT_ADDR, 0);
    println!("{:#?}", chain);
    chain.add_transaction("sender", "reciever", 50.0);
    println!("{:#?}", chain);
}
