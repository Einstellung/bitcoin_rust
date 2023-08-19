use crate::block::Block;
use num_bigint::BigUint;
use sha2::{Sha256, Digest};
use crate::utils;

// difficulty level
const TARGET_BITS: u16 = 3;
const MAT_NONCE: u32 = u32::MAX;

pub struct ProofOfWork<'a> {
    block: &'a Block,
    // pow hash should less than target value
    target: BigUint
}


impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a Block) -> Self {
        // 0000...1
        let mut target = BigUint::from(1u32);
        target = target << (256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: u32) -> Vec<u8> {
        let data = vec![
            &self.block.prev_block_hash,
            &self.block.data,
            &self.block.time_stamp.to_le_bytes() as &[u8],
            &TARGET_BITS.to_le_bytes(),
            &nonce.to_le_bytes(),
        ].concat();
        data
    } 

    // calculate pow then return right nonce and hash
    pub fn run(&self) -> (u32, Vec<u8>) {
        let mut nonce = 0;
        // define outside while and clear in the while is to 
        // avoid constant allocation of new memory space to imporve performance
        let mut hash = Vec::new();
        let mut hasher = Sha256::new();

        println!("Mining the block containing {}", utils::hex_string(&self.block.data));
        while nonce < MAT_NONCE {
            hash.clear();

            let data = self.prepare_data(nonce);
            hasher.update(data);
            hash = hasher.finalize_reset().to_vec();
            
            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        (nonce, hash)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();
        let hash_int = BigUint::from_bytes_be(&hash);

        hash_int < self.target
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    #[test]
    fn test() {
        let mut target = BigUint::from(1u32);
        target = target << (256 - 3);
        println!("target value result {}", target.bits());
    }
}