use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use std::fmt::Write;

pub struct Block {
    time_stamp: i64,
    // data is the actual valuable information containing in the block (serialize to binary)
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    // the hash of the block
    pub hash: Vec<u8>
}

// #[allow(unused)]
impl Block {
    pub fn new(data: &str, prev_block_hash: Vec<u8>) -> Block {
        let mut block = Block {
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            data: data.as_bytes().to_vec(),
            prev_block_hash,
            hash: Vec::new()
        };
        block.set_hash();
        block
    }

    // concatenate block values and use sha256 to set this block hash
    fn set_hash(&mut self) {
        // concat change from [[1,2,3], [4,5,6]] to [1,2,3,4,5,6]
        let headers = vec![
            &self.time_stamp.to_le_bytes() as &[u8],
            &self.data,
            &self.prev_block_hash
        ].concat();

        let mut hasher = Sha256::new();
        hasher.update(headers);
        self.hash = hasher.finalize().to_vec();
    }

    pub fn new_genesis_block() -> Block {
        Block::new("Genesis Block", vec![])
    }

    pub fn print_content(&self) {
        println!("Timestamp: {}", self.time_stamp);
        println!("Data: {}", String::from_utf8_lossy(&self.data));
        println!("Previous Bloch Hash: {}", hex_string(&self.prev_block_hash));
        println!("Hash {}", hex_string(&self.hash));
    }
}

// change byte code to hex string
fn hex_string(vec: &Vec<u8>) -> String {
    let mut s = String::new();
    for byte in vec {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}
