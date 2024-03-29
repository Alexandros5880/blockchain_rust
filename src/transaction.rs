use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: i32,
}