use sha2::{Sha256, Digest};

// change byte code to hex string
pub fn hex_string(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}

pub fn string_hex(s: &String) -> Vec<u8> {
    let res = hex::decode(s);
    match res {
        Ok(data) => data,
        Err(e) => {
            eprint!("Failed to exchange hex data to string, err: {}", e);
            Vec::new()
        }
    }
}

pub fn compute_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}