use crate::{block::Block, transaction::Transaction};

pub fn encode_block(block: &Block) -> Vec<u8> {
    serde_json::to_vec(block).unwrap()
}

pub fn decode_block(data: &[u8]) -> Block {
    serde_json::from_slice(data).unwrap()
}

pub fn encode_tx(tx: &Transaction) -> Vec<u8> {
    serde_json::to_vec(tx).unwrap()
}

pub fn decode_tx(data: &[u8]) -> Transaction {
    serde_json::from_slice(data).unwrap()
}
