mod block;
mod blockchain;
mod bcdb;
mod proofofwork;
mod utils;
mod cli;

use blockchain::Blockchain;
use cli::CLI;

fn main() {
    let blockchain = Blockchain::new();
    // blockchain.add_block("Send 1 BTC to Alice");
    // blockchain.add_block("Send 2 BTC to Bob");
    // blockchain.print_block();

    let mut cli = CLI::new(blockchain);
    cli.run();
}
