use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_id: String,
    pub listen_addr: String,
    pub genesis_path: String,
}

impl NodeConfig {
    pub fn load_default() -> Self {
        Self {
            node_id: "node-1".into(),
            listen_addr: "0.0.0.0:30303".into(),
            genesis_path: "genesis/genesis.json".into(),
        }
    }
}
