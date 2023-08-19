mod block;
mod blockchain;
mod proofofwork;
mod utils;

use blockchain::Blockchain;
use proofofwork::ProofOfWork;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Send 1 BTC to Alice");
    blockchain.add_block("Send 2 BTC to Bob");
    blockchain.print_block();

    for block in &blockchain.blocks {
        let pow = ProofOfWork::new(block);
        println!("Pow: {}", pow.validate());
        println!();
    }
}
