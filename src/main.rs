use blockchain::block::{Block, mine_block};
use blockchain::blockchain::{Blockchain, new_blockchain};
use blockchain::transaction;
// use blockchain::merkle_tree;
use chrono::Utc;
use pretty_env_logger;
use log::{info, warn};
// use serde_json;

fn main() {
    pretty_env_logger::init();

    warn!("In main!");
    // match transaction::build_transaction(0) {
    //     Option::None => println!("Failed creating transaction!"),
    //     Option::Some(tx) => {
    //         println!("Created {:?}", tx);
    //         println!("Hash of the transaction")
    //     }
    // }

    let example_transaction: &str = r#"
{
	"vin": [{
		"txid": "3f4fa19803dec4d6a84fae3821da7ac7577080ef75451294e71f9b20e0ab1e7b",
		"vout": 0,
		"scriptkey": "1",
		"sequence": 4294967295
	}],
	"vout": [{
		"n": 0,
		"address": "1",
		"scriptkey": "1",
		"value": 50.0,
		"sequence": 4294967295
	}]
}
"#;

    // Use ? when a function can return an error!!!
    let json_transaction: serde_json::Value = serde_json::from_str(example_transaction).unwrap();

    info!("Vin: {:?}", json_transaction["vin"]);
    info!("Vout: {:?}", json_transaction["vout"]);
    info!("Doesn't exist: {:?}", json_transaction["huh"]);
    
    let maybe_tx: Option<transaction::Transaction> = transaction::build_transaction(0);

    match maybe_tx {
        Option::None => println!("Failed creating transaction!"),
        Option::Some(tx) => {
            let _d: Vec<u8> = tx.hash_transaction();

            // println!("Created {:?}", tx);
            // println!("Hash of the transaction {:?} with size {}", d, d.len());
        }
    }

    let mut new_blockchain: Blockchain = new_blockchain();
    let example_block: Block = Block {
        id: 1,
        timestamp: Utc::now().timestamp(),
        prev_hash: String::from("0"),
        nonce: 0,
        hash: String::from("a1a61af0495d12012f3120de2ca4580b34e61ea859bdc9b74e1ec9617098ec4a")
    };

    let mined_block: Block = mine_block(example_block);
    
    match new_blockchain.add_block(mined_block) {
        Result::Ok(_) => info!("Successfully added block!"),
        Result::Err(_) => warn!("Couldn't add block!")
    };
}
