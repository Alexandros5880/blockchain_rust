use sha2::{Sha256, Digest}; // Sha512
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: usize,
    pub previous_hash: String,
}

impl Block {
    pub fn new(previous_hash: Option<String>, proof: usize) -> Block {
        Block {
            index: 0, // index will be set when added to chain
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            transactions: vec![],
            proof,
            previous_hash: previous_hash.unwrap_or_else(|| String::from("1")), // Set default value
        }
    }

    pub fn hash(&self) -> String {
        let block_string = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&block_string);
        format!("{:X}", hasher.finalize())
    }
}