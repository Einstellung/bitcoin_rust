use std::time::{SystemTime, UNIX_EPOCH};
use crate::{utils, proofofwork::ProofOfWork, transaction::Transaction};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub time_stamp: i64,
    pub transaction: Vec<Transaction>,
    pub prev_block_hash: Vec<u8>,
    // the hash of the block
    pub hash: Vec<u8>,
    // use in pow method hash + nonce < target value
    pub nonce: u32
}

// #[allow(unused)]
impl Block {
    pub fn new(transaction: Vec<Transaction>, prev_block_hash: Vec<u8>) -> Block {
        let mut block = Block {
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            transaction,
            prev_block_hash,
            hash: Vec::new(),
            nonce: 0
        };
        // use pow method to set hash
        // block.set_hash();
        let pow = ProofOfWork::new(&block);
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        block
    }

    pub fn new_genesis_block(coinbase: Vec<Transaction>) -> Block {
        Block::new(coinbase, vec![])
    }

    /**
     * We want all transactions in a block to be uniquely identified by a single hash.
     * To achieve this, we get hashes of each transaction, concatenate them, and get a 
     * hash of concatenated combination.
     */
    pub fn hash_transaction(&self) -> Vec<u8> {
        let mut tx_hashes = Vec::new();

        for tx in &self.transaction {
            tx_hashes.push(tx.id.clone());
        }

        let tx_hash = utils::compute_sha256(&tx_hashes.concat());
        tx_hash
    }

    pub fn print_content(&self) {
        println!("Timestamp: {}", self.time_stamp);
        // todo!("rewrite data");
        // println!("Data: {}", String::from_utf8_lossy(&self.data));
        println!("Previous Bloch Hash: {}", utils::hex_string(&self.prev_block_hash));
        println!("Hash {}", utils::hex_string(&self.hash));
    }

    pub fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Serialization error: {}", e);
                Vec::new()
            }
        }
    }

    pub fn deserialize(bytes: &Vec<u8>) -> Option<Block> {
        match bincode::deserialize(bytes.as_slice()) {
            Ok(block) => Some(block),
            Err(e) => {
                eprintln!("Deserialization error: {}", e);
                None
            } 
        }
    }
}
