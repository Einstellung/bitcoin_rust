use crate::bcdb::BlockchainDb;
use crate::block::Block;
use crate::proofofwork::ProofOfWork;
use leveldb::database::Database;

const DB_FILE: &str = "blockchain.db";

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
    pub fn new() -> Self {
        let mut db = BlockchainDb::new(DB_FILE);
        
        let tip;

        if let Some(last_hash) = BlockchainDb::read_db(&db, b"l") {
            tip = last_hash;
        } else {
            println!("No existing blockchain found. Creating a new one...");
            let genesis_block = Block::new_genesis_block();
            BlockchainDb::write_db(&mut db, genesis_block.hash.as_slice().clone(), &genesis_block.serialize());
            BlockchainDb::write_db(&mut db, b"l", &genesis_block.hash);
            tip = genesis_block.hash;
        }

        Blockchain { tip,  db }
    }

    pub fn add_block(&mut self, data: &str) {

        let last_hash = BlockchainDb::read_db(&self.db, b"l").unwrap();

        let last_block_serialize = BlockchainDb::read_db(&self.db, last_hash.clone().as_slice());

        let last_block = Block::deserialize(&last_block_serialize.unwrap()).unwrap();

        let pow = ProofOfWork::new(&last_block);

        if pow.validate() == true {
            let new_block = Block::new(data, last_hash.clone());
            BlockchainDb::write_db(&mut self.db, &new_block.hash.clone(), &new_block.serialize());
            BlockchainDb::write_db(&mut self.db, b"l", &new_block.hash.clone());
            self.tip = new_block.hash;
        } else {
            panic!("Pow validate error, stop to add new block for blockchain!")
        }
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