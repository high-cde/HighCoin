use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub chain_id: String,
    pub height: u64,
    pub prev_hash: Vec<u8>,
    pub state_root: Vec<u8>,
    pub tx_root: Vec<u8>,
    pub timestamp: i64,
    pub validator_id: String,
    pub validator_sig: Vec<u8>,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}

impl Block {
    pub fn empty(height: u64) -> Self {
        Self {
            header: BlockHeader {
                chain_id: "HighBridgeChain-1".into(),
                height,
                prev_hash: vec![],
                state_root: vec![],
                tx_root: vec![],
                timestamp: Utc::now().timestamp(),
                validator_id: "node-1".into(),
                validator_sig: vec![],
                version: 1,
            },
            txs: vec![],
        }
    }
}
