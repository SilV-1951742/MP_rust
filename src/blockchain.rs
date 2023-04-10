use chrono::Utc;
use std::fmt;
use crate::block::{Block, leading_zeros};
use log::warn;
use hex;

const DIFFICULTY_PREFIX: u32 = 4;

#[derive(Debug, Clone)]
pub struct BlockAddError;

impl fmt::Display for BlockAddError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while adding block to chain.")
    }
}

#[derive(Debug, Clone)]
pub struct BlockValidationError;

impl fmt::Display for BlockValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while validating a block.")
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>
}

pub fn new_blockchain() -> Blockchain {
    let mut new_blockchain: Blockchain = Blockchain {blocks: vec![]};
    new_blockchain.construct_genesis();
    return new_blockchain;
}


impl Blockchain {
    fn construct_genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            prev_hash: String::from("0"),
            nonce: 0,
            hash: String::from("0")
        };

        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), BlockAddError> {
        let last_block = self.blocks.last().expect("there is at least one block");
        
        match self.validate_block(&block, last_block) {
            Result::Ok(_) => {
                warn!("Block validated!");
                self.blocks.push(block);
                return Result::Ok(());
            },
            Result::Err(_) => {
                warn!("Couldn't validate block!");
                return Result::Err(BlockAddError);
            }
        }
    }

    fn validate_block(&self, block: &Block, previous_block: &Block) -> Result<(), BlockValidationError> {
        if block.prev_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return Result::Err(BlockValidationError);
        } else if leading_zeros(hex::decode(&block.hash).unwrap()) < DIFFICULTY_PREFIX {
            warn!("block with id: {} has invalid difficulty", block.id);
            return Result::Err(BlockValidationError);
        } else if block.id != previous_block.id + 1 {
            warn!("block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id);
            return Result::Err(BlockValidationError);
        } else if hex::encode(block.hash_block()) != block.hash {
            warn!("block with id: {} has invalid hash", block.id);
            println!("Got {} and expected {}", block.hash, hex::encode(block.hash_block()));
            return Result::Err(BlockValidationError);
        }
        return Result::Ok(());
    }
}
