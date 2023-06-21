use std::collections::HashMap;

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
// use bincode;


// #[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
// pub struct OutPoint {
//     /// The referenced transaction's txid.
//     pub txid: String,
//     /// The index of the referenced output in its transaction's vout.
//     pub vout: u32,
// }

// impl OutPoint {
//     /// Creates a new [`OutPoint`].
//     #[inline]
//     pub fn new(txid: String, vout: u32) -> OutPoint {
//         OutPoint { txid, vout }
//     }

//     #[inline]
//     pub fn null() -> OutPoint {
//         OutPoint {
//             txid: String::from(""),
//             vout: u32::max_value(),
//         }
//     }

//     #[inline]
//     pub fn is_null(&self) -> bool {
//         *self == OutPoint::null()
//     }
// }

// impl Default for OutPoint {
//     fn default() -> Self {
//         OutPoint::null()
//     }
// }


// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub struct TransactionInput {
//     pub txid: String,
//     pub vout: u32,
//     pub scriptkey: String,
// }

// impl Default for TransactionInput {
//     fn default() -> Self {
//         TransactionInput {
//             txid: String::from(""),
//             vout: u32::MAX,
//             scriptkey: String::from("")
//         }
//     }
// }


// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub struct TransactionOutput {
//     pub n: u32,
//     pub address: String,
//     pub scriptkey: String,

//     pub value: u32,
// }

// impl Default for TransactionOutput {
//     fn default() -> Self {
//         TransactionOutput {
//             n: u32::MAX,
//             address: String::from(""),
//             scriptkey: String::from(""),
//             value: u32::MAX
//         }
//     }
// }


// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub struct Transaction {
//     pub vin: Vec<TransactionInput>,
//     pub vout: Vec<TransactionOutput>,
//     // transaction_hash: Vec<u8>
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Transaction {
    transaction_id: String,
    input_transaction_id: String,
    property_id: String,
    buyer_id: String,
    seller_id: String,
    signatures: HashMap<String, String>, // A mapping of party IDs to their digital signatures
}

pub fn build_transaction() -> Option<Transaction> {
    // let mut tx: Transaction = Transaction {
    //     vin: Vec::new(),
    //     vout: Vec::new(),
    // };
    
    // tx.vin.push(TransactionInput {
    //     txid: String::from(""),
    //     vout: 0,
    //     scriptkey: String::from("")
    // });

    // tx.vout.push(TransactionOutput {
    //     n: 0,
    //     address: String::from(""),
    //     scriptkey: String::from(""),
    //     value: 0
    // });

    let tx: Transaction = Transaction {
        transaction_id: String::new(),
        input_transaction_id: String::new(),
        property_id: String::new(),
        buyer_id: String::new(),
        seller_id: String::new(),
        signatures: HashMap::new()
    };
    
    Some(tx)
}

impl Transaction {
    pub fn new(input_transaction_id: String, property_id: String, buyer_id: String, seller_id: String, signatures: HashMap<String, String>) -> Self {
        let id_str = &format!("{}{}{}{}{}", input_transaction_id, property_id, buyer_id, seller_id, serde_json::to_string(&signatures).unwrap());
        let mut hasher = Sha256::new();
        hasher.update(id_str);
        let transaction_id = String::from_utf8_lossy(hasher.finalize().as_slice()).to_string();
        Self {transaction_id,
              input_transaction_id,
              property_id,
              buyer_id,
              seller_id,
              signatures,
        }
    }
    
    pub fn hash_transaction(&self) -> Vec<u8> {
        let tx_bytes: Vec<u8> = bincode::serialize(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(tx_bytes);
        return hasher.finalize().as_slice().to_owned()
    }

    pub fn txid(&self) -> Vec<u8> {
        let cloned_tx: Transaction = self.clone();
        return cloned_tx.hash_transaction();
    }
}
