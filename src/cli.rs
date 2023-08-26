use std::env;

use crate::{blockchain::Blockchain, transaction::new_utxo_transaction};

pub struct CLI;

impl CLI {
    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            self.print_usage();
            return;
        }

        let command = &args[1];
        match command.as_str() {
            "createblockchain" => {
                if args.len() != 4 {
                    println!("Usage: createblockchain -address ADDRESS - Create a blockchain and send genesis block reward to ADDRESS");
                } else {
                    self.create_blockchain(&args[3]);
                }                 
            },
            "getbalance" => {
                if args.len() != 4 {
                    println!("Usage: getbalance -address ADDRESS - Get balance of ADDRESS");
                } else {
                    self.get_balance(&args[3]);
                }  
            },
            "send" => {
                if args[3].is_empty() || args[5].is_empty() || args[7].is_empty(){
                    println!("Usage: send -from FROM -to TO -amount AMOUNT - Send AMOUNT of coins from FROM address to TO");
                } else {
                    let from = &args[3];
                    let to = &args[5];
                    let amount = args[7].parse::<i32>().expect("Invalid amount");
                    self.mine_block(from, to, amount);
                }               
            },
            "printchain" => {
                self.print_chain();
            },
            _ => {
                self.print_usage();
            },
        }
    }

    fn print_usage(&self) {
        println!("Usage:");
        println!("  getbalance -address ADDRESS - Get balance of ADDRESS");
        println!("  createblockchain -address ADDRESS - Create a blockchain and send genesis block reward to ADDRESS");
        println!("  printchain - Print all the blocks of the blockchain");
        println!("  send -from FROM -to TO -amount AMOUNT - Send AMOUNT of coins from FROM address to TO");
    }

    pub fn mine_block(&self, from: &str, to: &str, amount: i32) {
        let mut bc = Blockchain::new(from);
        let tx = new_utxo_transaction(from, to, amount, &bc);
        bc.mine_block(vec![tx]);
        println!("Success!");
    }

    pub fn create_blockchain(&self, address: &str) {
        Blockchain::new(address);
        println!("Done");
    }

    pub fn get_balance(&self, address: &str) {
        let bc = Blockchain::new(address);
        let mut balance = 0;

        let utxos = bc.find_utxo(address);

        for utxo in utxos {
            balance += utxo.value;
        }

        println!("Balance of {}: {}", address, balance);
    }

    fn print_chain(&self) {
        let bc = Blockchain::new("");
        bc.print_block();
    }
}