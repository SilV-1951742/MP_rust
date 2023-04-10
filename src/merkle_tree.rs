use std::collections::HashMap;
use crate::transaction::Transaction;

#[derive(Debug)]
enum Child {
    Leaf([u8; 64]),
    Node(MerkleNode)
}

#[derive(Debug)]
struct MerkleNode {
    // tx: transaction::Transaction
    // hash: or
    left: Box<Child>,
    right: Box<Child>,
    hash: [u8; 64]
}

pub struct MerkleTree {
    leaves: HashMap<[u8; 64], Transaction>
}

impl MerkleTree {
    // pub fn build_tree(&self, txs: Vec<Transaction>) -> MerkleTree {
    //     for tx in txs {
    //         tx.ha
    //     }
    // }
}
