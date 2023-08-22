use std::env;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions, ReadOptions};
use leveldb::database::Database;
use byteorder::{ByteOrder, LittleEndian};

pub struct BlockchainDb;

impl BlockchainDb {
    pub fn new(local_path: &str) -> Database<i32> {
        let mut dir_path = env::current_dir().unwrap();
        dir_path.push(local_path);

        let mut opts = Options::new();
        // if there isn't exist db file then create it
        opts.create_if_missing = true;

        let database = match Database::open(&dir_path, opts) {
            Ok(db) => db,
            Err(e) => panic!("Failed to open database: {:?}", e)
        };
        database
    }

    pub fn write_db(db: &mut Database<i32>, key: &[u8], val: &[u8]) {
        let write_opts = WriteOptions::new();
        match db.put(write_opts, from_u8(key), val) {
            Ok(_) => (),
            Err(e) => panic!("Failed write block in database: {:?} ",e)
        }
    }

    pub fn read_db(db: &Database<i32>, key: &[u8]) -> Option<Vec<u8>> {
        let read_options = ReadOptions::new();
        match db.get(read_options, from_u8(key)) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to read read block from database: {:?}", e);
                None
            }
        }
    }
}

fn from_u8(key: &[u8]) -> i32 {
    let mut buffer = [0u8; 4];
    let mut key_truncate = key;
    if key_truncate.len() >= 4 {
        key_truncate = &key_truncate[key_truncate.len() - 4..];
    }
    buffer[4 - key_truncate.len()..].copy_from_slice(key_truncate);

    let res = LittleEndian::read_i32(&buffer);
    res
}