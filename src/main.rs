mod block;
mod blockchain;
mod bcdb;
mod proofofwork;
mod utils;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Send 1 BTC to Alice");
    blockchain.add_block("Send 2 BTC to Bob");
    blockchain.print_block();

}
