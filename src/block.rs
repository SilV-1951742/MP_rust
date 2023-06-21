use log::info;
// use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt::Write;
use crate::transaction::Transaction;
use chrono::Utc;


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub transactions: Vec<Transaction>
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
    zeros
}

pub fn encode_hex(bytes: Vec<u8>) -> String {
    let mut s: String = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn mine_block(block: Block, difficulty: u32) -> Block {
    let mut hash: String = encode_hex(block.hash_block());
    let mut tmp_block: Block = Block {
        hash: String::from("0"),
        prev_hash: block.prev_hash,
        timestamp: block.timestamp,
        nonce: block.nonce,
        transactions: block.transactions
    };
    
    while leading_zeros(hex::decode(&hash).unwrap()) < difficulty {
        tmp_block.nonce += 1;
        hash = encode_hex(tmp_block.hash_block());
    }
    info!("Nonce: {}", tmp_block.nonce);
    info!("{}", hash);
    tmp_block.hash = hash;
    tmp_block
}

impl Block {
    pub fn new(prev_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp =  Utc::now().timestamp();
        let nonce = 0;
        let hash = "".to_string();

        Self {hash,
              prev_hash,
              timestamp,
              nonce,
              transactions,
        }
    }
    
    pub fn hash_block(&self) -> Vec<u8> {
        let mut block_bytes: Vec<u8> = vec![];
        
        block_bytes.append(&mut bincode::serialize(&self.prev_hash).unwrap());
        block_bytes.append(&mut bincode::serialize(&self.timestamp).unwrap());
        block_bytes.append(&mut bincode::serialize(&self.nonce).unwrap());
        block_bytes.append(&mut bincode::serialize(&self.transactions).unwrap());
        
        let mut hasher = Sha256::new();
        hasher.update(block_bytes);
        hasher.finalize().as_slice().to_owned()
    }
}
