use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
// use bincode;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct TransactionInput {
    outputpoint_hash: String,
    outputpoint_idx: u32,
    scriptkey: String,

    amount: u128,
    
    sequence: u128,
    confirmations: u128,

    spendable: bool,
    solvable: bool
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct TransactionOutput {
    idx: u32,
    address: String,
    scriptkey: String,

    amount: u128,
    
    sequence: u128,
    confirmations: u128,

    spendable: bool,
    solvable: bool
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    transaction_index: u128,
    // transaction_hash: Vec<u8>
}

pub fn build_transaction(index: u128) -> Option<Transaction> {
    let mut tx: Transaction = Transaction {
        inputs: Vec::new(),
        outputs: Vec::new(),
        transaction_index: index,
    };
    
    tx.inputs.push(TransactionInput {
        outputpoint_hash: "".to_string(),
        outputpoint_idx: 0,
        scriptkey: "".to_string(),
        amount: 0,
        sequence: 0,
        confirmations: 0,
        spendable: false,
        solvable: false
    });

    tx.outputs.push(TransactionOutput {
        idx: 0,
        address: "".to_string(),
        scriptkey: "".to_string(),
        amount: 0,
        sequence: 0,
        confirmations: 0,
        spendable: false,
        solvable: false
    });

    // match hash_transaction(tx) {
    //     (_, Option::None) => {
    //         return None
    //     },
    //     (o_tx, Option::Some(d)) => {
    //         tx = o_tx;
    //         tx.transaction_hash = d.as_ref().to_vec();
    //         return Some(tx)
    //     }
    // }

    Some(tx)
}

impl Transaction {
    pub fn hash_transaction(&self) -> Vec<u8> {
        let tx_bytes: Vec<u8> = bincode::serialize(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(tx_bytes);
        return hasher.finalize().as_slice().to_owned()
    }
}


