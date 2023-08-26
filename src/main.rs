mod block;
mod blockchain;
mod bcdb;
mod proofofwork;
mod utils;
mod cli;
mod transaction;

use cli::CLI;

fn main() {

    let cli = CLI;
    cli.run();

    // cli.create_blockchain("Pedro");

    // cli.mine_block("Pedro", "Ivan", 10);
    // cli.get_balance("Ivan");
    // cli.get_balance("Pedro");
    // cli.mine_block("Ivan", "Pedro", 1);
    // cli.get_balance("Ivan");
    // cli.get_balance("Pedro");
}