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


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
    pub scriptkey: String,
}

impl Default for TransactionInput {
    fn default() -> Self {
        TransactionInput {
            txid: String::from(""),
            vout: u32::MAX,
            scriptkey: String::from("")
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TransactionOutput {
    pub n: u32,
    pub address: String,
    pub scriptkey: String,

    pub value: u32,
}

impl Default for TransactionOutput {
    fn default() -> Self {
        TransactionOutput {
            n: u32::MAX,
            address: String::from(""),
            scriptkey: String::from(""),
            value: u32::MAX
        }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Transaction {
    pub vin: Vec<TransactionInput>,
    pub vout: Vec<TransactionOutput>,
    // transaction_hash: Vec<u8>
}


pub fn build_transaction() -> Option<Transaction> {
    let mut tx: Transaction = Transaction {
        vin: Vec::new(),
        vout: Vec::new(),
    };
    
    tx.vin.push(TransactionInput {
        txid: String::from(""),
        vout: 0,
        scriptkey: String::from("")
    });

    tx.vout.push(TransactionOutput {
        n: 0,
        address: String::from(""),
        scriptkey: String::from(""),
        value: 0
    });
    
    Some(tx)
}

impl Transaction {
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
