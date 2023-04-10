use log::info;
// use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt::Write;

const DIFFICULTY_PREFIX: u32 = 4;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub nonce: u64
}

pub fn leading_zeros(hash: Vec<u8>) -> u32 {
    let mut zeros: u32 = 0;

    for c in hash {
        match c.leading_zeros() {
            0 => break,
            8 => zeros += 8,
            z => { zeros += z;
                   break; }
        };
    }

    return zeros
}

pub fn encode_hex(bytes: Vec<u8>) -> String {
    let mut s: String = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    return s
}

pub fn mine_block(block: Block) -> Block {
    let mut hash: String = encode_hex(block.hash_block());;
    let mut tmp_block: Block = Block {
        id: block.id,
        hash: String::from("0"),
        prev_hash: block.prev_hash,
        timestamp: block.timestamp,
        nonce: block.nonce
    };
    
    while leading_zeros(hex::decode(&hash).unwrap()) < DIFFICULTY_PREFIX {
        tmp_block.nonce += 1;
        hash = encode_hex(tmp_block.hash_block());
    }
    info!("Nonce: {}", tmp_block.nonce);
    info!("{}", hash);
    tmp_block.hash = hash;
    return tmp_block
}

impl Block {
    pub fn hash_block(&self) -> Vec<u8> {
        let mut block_bytes: Vec<u8> = vec![];
        
        block_bytes.append(&mut bincode::serialize(&self.id).unwrap());
        block_bytes.append(&mut bincode::serialize(&self.prev_hash).unwrap());
        block_bytes.append(&mut bincode::serialize(&self.timestamp).unwrap());
        block_bytes.append(&mut bincode::serialize(&self.nonce).unwrap());
        
        let mut hasher = Sha256::new();
        hasher.update(block_bytes);
        return hasher.finalize().as_slice().to_owned()
    }
}
