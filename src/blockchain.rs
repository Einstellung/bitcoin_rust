use std::collections::HashMap;
use crate::bcdb::BlockchainDb;
use crate::block::Block;
use crate::proofofwork::ProofOfWork;
use crate::transaction::{new_coinbase_tx, Transaction, TXOutput};
use crate::utils;
use leveldb::database::Database;

const DB_FILE: &str = "blockchain.db";

const GENESIS_COINBASE_DATA: &str = "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

/**
 * blockchain db persistence struct
 * 32-byte block-hash -> Block Structure (serialized)
 * 'l' -> the hash of the last block in chain 
 */

pub struct Blockchain {
    // last block hash
    tip: Vec<u8>,
    // pub blocks: Vec<Block>,
    db: Database<i32>
}

impl Blockchain {
    pub fn new(address: &str) -> Self {
        let mut db = BlockchainDb::new(DB_FILE);
        
        let tip;

        if let Some(last_hash) = BlockchainDb::read_db(&db, b"l") {
            tip = last_hash;
        } else if address == "" {
            panic!("Please create blockchian first");
        } else {
            println!("No existing blockchain found. Creating a new one...");
            let coinbase = new_coinbase_tx(address, GENESIS_COINBASE_DATA);
            let genesis_block = Block::new_genesis_block(vec![coinbase]);
            BlockchainDb::write_db(&mut db, genesis_block.hash.as_slice().clone(), &genesis_block.serialize());
            BlockchainDb::write_db(&mut db, b"l", &genesis_block.hash);
            tip = genesis_block.hash;
        }

        Blockchain { tip,  db }
    }

    pub fn mine_block(&mut self, transaction: Vec<Transaction>) {

        let last_hash = BlockchainDb::read_db(&self.db, b"l").unwrap();

        let last_block_serialize = BlockchainDb::read_db(&self.db, last_hash.clone().as_slice());

        let last_block = Block::deserialize(&last_block_serialize.unwrap()).unwrap();

        let pow = ProofOfWork::new(&last_block);

        if pow.validate() == true {
            let new_block = Block::new(transaction, last_hash.clone());
            BlockchainDb::write_db(&mut self.db, &new_block.hash.clone(), &new_block.serialize());
            BlockchainDb::write_db(&mut self.db, b"l", &new_block.hash.clone());
            self.tip = new_block.hash;
        } else {
            panic!("Pow validate error, stop to add new block for blockchain!")
        }
    }

    // return a list of transactions containing unspent outputs
    pub fn find_unspend_transaction(&self, address: &str) -> Vec<Transaction> {
        let mut unspend_txs: Vec<Transaction> = Vec::new();

        // spent transaction output
        // transaction id -> transaction vout index
        let mut spent_txo: HashMap<String, Vec<i32>> = HashMap::new();

        let mut blockchain_iterator = BlockchainIterator {
            prev_block_hash: self.tip.clone(),
            db: &self.db
        };

        while let Some(block) = blockchain_iterator.next() {
            for tx in block.transaction {
                let txid = utils::hex_string(&tx.id);
    
                for (tx_output_index, tx_output) in tx.vout.iter().enumerate() {
    
                    if Blockchain::is_spent_output(tx_output_index, &spent_txo, &txid) {
                        continue;
                    }
    
                    if tx_output.can_be_unlocked_with(address) {
                        unspend_txs.push(tx.clone());
                    }
    
                    if tx.is_coinbase() == false {
                        // gather all inputs that could unlock outputs locked with the provided address
                        for tx_input in &tx.vin {
                            // if the input could unlock output means this utxo has been spent
                            if tx_input.can_unlock_output_with(address) {

                                let tx_input_id = utils::hex_string(&tx_input.txid);

                                spent_txo.entry(tx_input_id)
                                        .or_insert(Vec::new())
                                        .push(tx_input.vout);
                            } 
                        }
                    }
                }
            }
        }
        unspend_txs
    }

    // find and returns all unspent outputs
    pub fn find_utxo(&self, address: &str) -> Vec<TXOutput> {
        let mut utxos: Vec<TXOutput> = vec![];
        let unspent_transactions = self.find_unspend_transaction(address);

        for tx in unspent_transactions {
            for out in tx.vout {
                if out.can_be_unlocked_with(address) {
                    utxos.push(out);
                }
            }
        }

        utxos
    }

    // find all unspent outputs and ensure that they store enough value
    pub fn find_spendable_outputs(&self, address: &str, amount: i32) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        // find fn is wrong sometimes
        let unspent_transactions = self.find_unspend_transaction(address);
        let mut accumulated = 0;

        'Work:
        for unspent_transaction in unspent_transactions {
            let txid = utils::hex_string(&unspent_transaction.id);

            for (tx_output_index, tx_output) in unspent_transaction.vout.iter().enumerate() {
                if tx_output.can_be_unlocked_with(address) && accumulated < amount {
                    accumulated += tx_output.value;

                    unspent_outputs.entry(txid.clone())
                                    .or_insert(Vec::new())
                                    .push(tx_output_index as i32);
                }

                if accumulated >= amount {
                    break 'Work;
                }
            }
        }
        
        (accumulated, unspent_outputs)
    }

    pub fn print_block(&self) {
        let mut blockchain_iterator = BlockchainIterator {
            prev_block_hash: self.tip.clone(),
            db: &self.db
        };

        while let Some(block) = blockchain_iterator.next() {
            block.print_content();
            println!("------------");
        }
    }

    // If the input transaction has already referenced the output transaction. It means
    // that output transaction has been spent, and it will not be considered.
    fn is_spent_output(tx_output_index: usize, spent_txo: &HashMap<String, Vec<i32>>, txid: &String) -> bool {
        if let Some(spent_outs) = spent_txo.get(txid) {
            for spent_out in spent_outs {
                if spent_out.clone() == tx_output_index as i32 {
                    return true;
                }
            }
        }
        false
    }
}

struct BlockchainIterator<'a> {
    prev_block_hash: Vec<u8>,
    db: &'a Database<i32>
}

impl <'a>BlockchainIterator<'a> {
    fn next(&mut self) -> Option<Block> {
        if let Some(encode_block) = BlockchainDb::read_db(&self.db, self.prev_block_hash.as_slice()) {
            let block = Block::deserialize(&encode_block).unwrap();
            self.prev_block_hash = block.prev_block_hash.clone();
            Some(block)
        } else {
            None
        }
    }
}