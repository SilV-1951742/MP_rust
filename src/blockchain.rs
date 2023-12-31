use std::fmt;
use crate::block::{Block, leading_zeros, mine_block};
use log::{warn, error};
use hex;
use itertools::Itertools;
use crate::transaction::Transaction;
use chrono::Utc;


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

#[derive(Debug, Clone)]
pub struct ChainValidationError;

impl fmt::Display for ChainValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while validating a chain.")
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u32,
    pub mempool: Vec<Transaction>
}

pub fn new_blockchain(diff: u32) -> Blockchain {
    let mut new_blockchain: Blockchain = Blockchain {
        blocks: vec![],
        difficulty: diff,
        mempool: vec![]
    };
    new_blockchain.construct_genesis();
    new_blockchain
}


impl Blockchain {
    fn construct_genesis(&mut self) {
        let mut txs: Vec<Transaction> = vec![];
        let genesis_transaction: &str = r#"
{
        "transaction_id": "1",
        "input_transaction_id": "0",
        "property_id": "abcdef",
        "buyer_id": "02eaf53b6f60206010e866d707c28e41f18e1d7076105a8d2d9952bf0bacf54762",
        "seller_id": "025e5ecbee213695b7ba9d01cf82188299627b054bc9fff368c6fe3bc24c66dc4a",
        "signatures": {"buyer1":"aaaaa","buyer2":"bbbbb"}
}
"#;
        let new_transaction: Transaction = serde_json::from_str(genesis_transaction).unwrap();
        txs.push(new_transaction);
        
        let genesis_block = Block {
            timestamp: Utc::now().timestamp(),
            prev_hash: String::from("0"),
            nonce: 0,
            hash: String::from("0"),
            transactions: txs
        };

        let mined_genesis: Block = mine_block(genesis_block, self.difficulty);
        self.blocks.push(mined_genesis);
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), BlockAddError> {
        let previous_block: &Block = self.blocks.last().expect("There is a least one block");
        match self.validate_block(&block, previous_block) {
            Result::Ok(_) =>{ 
                self.blocks.push(block);
                Result::Ok(())
            },
            Result::Err(_) => {
                error!("Couldn't validate block!");
                Result::Err(BlockAddError)
            }
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.push(transaction);
    }

    fn validate_block(&self, block: &Block, previous_block: &Block) -> Result<(), BlockValidationError> {
        if block.prev_hash != previous_block.hash {
            warn!("block {} has wrong previous hash", block.hash);
            return Result::Err(BlockValidationError);
        } else if leading_zeros(hex::decode(&block.hash).expect("Can't decode hex string!")) < self.difficulty {
            warn!("block {} has invalid difficulty", block.hash);
            return Result::Err(BlockValidationError);
        } else if hex::encode(block.hash_block()) != block.hash {
            warn!("block {} has invalid hash", block.hash);
            println!("Got {} and expected {}", block.hash, hex::encode(block.hash_block()));
            return Result::Err(BlockValidationError);
        }
        Result::Ok(())
    }

    pub fn validate_chain(&self) -> Result<u32, ChainValidationError> {
        for (block1, block2) in self.blocks.iter().tuple_windows() {
            match self.validate_block(block2, block1) {
                Result::Ok(_) => continue,
                Result::Err(_) => {
                    error!("Couldn't validate block {}!", block2.hash);
                    return Result::Err(ChainValidationError);
                }
            }
        }
        Result::Ok(u32::try_from(self.blocks.len()).expect("Can't cast usize to u32"))
    }

    pub fn update_chain(self, remote: Blockchain) -> Blockchain {
        match (self.validate_chain(), remote.validate_chain()) {
            (Result::Ok(l1), Result::Ok(l2)) => {
                if l1 >= l2 {
                    return Blockchain {blocks: self.blocks, difficulty: self.difficulty, mempool: self.mempool}
                } else {
                    return Blockchain {blocks: remote.blocks, difficulty: remote.difficulty, mempool: remote.mempool}
                }
            },
            (Result::Err(_), Result::Err(_)) => panic!("Both chains are invalid!"),
            (Result::Err(_), _) => {
                return Blockchain {blocks: remote.blocks, difficulty: remote.difficulty, mempool: remote.mempool}
            },
            (_, Result::Err(_)) => {
                return Blockchain {blocks: self.blocks, difficulty: self.difficulty, mempool: self.mempool}
            }
        };
    }
}
