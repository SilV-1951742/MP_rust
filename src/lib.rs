pub mod transaction;
// pub mod merkle_tree;
pub mod block;
pub mod blockchain;

#[cfg(test)]
mod tests {
    use super::block::{Block, mine_block};
    use super::blockchain::{Blockchain, new_blockchain};
    use super::transaction;
    use chrono::Utc;
    #[cfg(test)]
    use std::{println as info, println as warn};

    #[test]
    fn test_transaction() {
        let example_transaction: &str = r#"
{
	"vin": [{
		"txid": "3f4fa19803dec4d6a84fae3821da7ac7577080ef75451294e71f9b20e0ab1e7b",
		"vout": 0,
		"scriptkey": "1",
	}],
	"vout": [{
		"n": 0,
		"address": "1",
		"scriptkey": "1",
		"value": 50.0,
	}]
}
"#;

        // Use ? when a function can return an error!!!
        // let json_transaction: serde_json::Value = serde_json::from_str(example_transaction).unwrap();

        let maybe_tx: Option<transaction::Transaction> = transaction::build_transaction();

        match maybe_tx {
            Option::None => panic!("Failed creating transaction!"),
            Option::Some(tx) => {
                let d: Vec<u8> = tx.hash_transaction();

                println!("Created {:?}", tx);
                println!("Hash of the transaction {:?} with size {}", d, d.len());
            }
        }
    }
    
    #[test]
    fn test_blockchain() {
        let mut new_blockchain: Blockchain = new_blockchain();
        let example_block1: Block = Block {
            id: 1,
            timestamp: Utc::now().timestamp(),
            prev_hash: String::from("0"),
            nonce: 0,
            hash: String::from(""),
            transactions: vec![]
        };

        let mined_block1: Block = mine_block(example_block1, 4);
        match new_blockchain.add_block(mined_block1) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };

        let example_block2: Block = Block {
            id: 2,
            timestamp: Utc::now().timestamp(),
            prev_hash: new_blockchain.blocks.last().unwrap().hash.clone(),
            nonce: 0,
            hash: String::from(""),
            transactions: vec![]
        };
    
        let mined_block2: Block = mine_block(example_block2, 4);
        match new_blockchain.add_block(mined_block2) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };

        let example_block3: Block = Block {
            id: 3,
            timestamp: Utc::now().timestamp(),
            prev_hash: new_blockchain.blocks.last().unwrap().hash.clone(),
            nonce: 0,
            hash: String::from(""),
            transactions: vec![]
        };

        let mined_block3: Block = mine_block(example_block3, 4);
        match new_blockchain.add_block(mined_block3) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };
    
        let example_block4: Block = Block {
            id: 4,
            timestamp: Utc::now().timestamp(),
            prev_hash: new_blockchain.blocks.last().unwrap().hash.clone(),
            nonce: 0,
            hash: String::from(""),
            transactions: vec![]
        };

        let mined_block4: Block = mine_block(example_block4, 4);
        match new_blockchain.add_block(mined_block4) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };

        match new_blockchain.validate_chain() {
            Result::Ok(l) => info!("Chain is ok! Checked {} blocks", l),
            Result::Err(_) => panic!("Invalid chain!")
        };

        assert_eq!(5, new_blockchain.blocks.len());
    }
}
