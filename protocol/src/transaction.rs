use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_type: String,
    pub from: String,
    pub payload: HashMap<String, serde_json::Value>,
    pub fee: i64,
    pub nonce: u64,
    pub sig: Vec<u8>,
}
