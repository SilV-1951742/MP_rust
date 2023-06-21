pub mod transaction;
// pub mod merkle_tree;
pub mod block;
pub mod blockchain;
pub mod p2p_nw;

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
        "property_id": "abcdef",
        "buyer_id": "buyer1",
        "seller_id": "buyer2",
        "signatures": {"buyer1":"aaaaa","buyer2":"bbbbb"}
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
        let mut new_blockchain: Blockchain = new_blockchain(4);
        let example_block1: Block = Block(1, String::from("0"), vec![]);

        let mined_block1: Block = mine_block(example_block1, 4);
        match new_blockchain.add_block(mined_block1) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };

        let example_block1: Block = Block(2, new_blockchain.blocks.last().unwrap().hash.clone(), vec![]);
    
        let mined_block2: Block = mine_block(example_block2, 4);
        match new_blockchain.add_block(mined_block2) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };

        let example_block1: Block = Block(3, new_blockchain.blocks.last().unwrap().hash.clone(), vec![]);

        let mined_block3: Block = mine_block(example_block3, 4);
        match new_blockchain.add_block(mined_block3) {
            Result::Ok(_) => info!("Successfully added block!"),
            Result::Err(_) => panic!("Couldn't add block!")
        };

        let example_block1: Block = Block(4, new_blockchain.blocks.last().unwrap().hash.clone(), vec![]);

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
