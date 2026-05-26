#!/usr/bin/env bash
set -euo pipefail

ROOT="$HOME/System.zdos/projects/high-bridgechain"

echo "[*] Working in: $ROOT"
cd "$ROOT"

echo "[1] Fix protocol/Cargo.toml"
cat > "$ROOT/protocol/Cargo.toml" << 'EOF'
[package]
name = "protocol"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
EOF

echo "[2] Fix rpc/Cargo.toml"
cat > "$ROOT/rpc/Cargo.toml" << 'EOF'
[package]
name = "rpc"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bytes = "1.0"
EOF

echo "[3] Fix rpc/src/server.rs"
mkdir -p "$ROOT/rpc/src"
cat > "$ROOT/rpc/src/server.rs" << 'EOF'
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_rpc_server(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("[RPC] Listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];

            loop {
                let n = match socket.read(&mut buf[..]).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("[RPC] read error: {:?}", e);
                        return;
                    }
                };

                let data = &buf[..n];
                println!("[RPC] Received: {:?}", data);

                if let Err(e) = socket.write_all(data).await {
                    eprintln!("[RPC] write error: {:?}", e);
                    return;
                }
            }
        });
    }
}
EOF

echo "[4] Fix tools/block-explorer-lite layout"
BLE="$ROOT/tools/block-explorer-lite"
mkdir -p "$BLE/src"
# se esiste un main.rs fuori da src, spostalo
if [ -f "$BLE/main.rs" ] && [ ! -f "$BLE/src/main.rs" ]; then
  mv "$BLE/main.rs" "$BLE/src/main.rs"
fi

cat > "$BLE/Cargo.toml" << 'EOF'
[package]
name = "block-explorer-lite"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "block-explorer-lite"
path = "src/main.rs"
EOF

echo "[5] Fix node module layout"
NODE="$ROOT/node"

mkdir -p "$NODE/src/consensus" "$NODE/src/network" "$NODE/src/state" "$NODE/src/zlang_vm"

# sposta moduli se sono ancora fuori da src
[ -f "$NODE/config.rs" ] && mv "$NODE/config.rs" "$NODE/src/config.rs" || true
[ -f "$NODE/consensus/mod.rs" ] && mv "$NODE/consensus/mod.rs" "$NODE/src/consensus/mod.rs" || true
[ -f "$NODE/network/mod.rs" ] && mv "$NODE/network/mod.rs" "$NODE/src/network/mod.rs" || true
[ -f "$NODE/state/mod.rs" ] && mv "$NODE/state/mod.rs" "$NODE/src/state/mod.rs" || true
[ -f "$NODE/zlang_vm/mod.rs" ] && mv "$NODE/zlang_vm/mod.rs" "$NODE/src/zlang_vm/mod.rs" || true

echo "[6] Rewrite node/src/main.rs"
cat > "$NODE/src/main.rs" << 'EOF'
pub mod config;
pub mod network;
pub mod consensus;
pub mod state;
pub mod zlang_vm;

use config::NodeConfig;
use network::Network;
use consensus::ConsensusEngine;
use state::State;
use zlang_vm::ZLangVm;
use rpc::server::start_rpc_server;

#[tokio::main]
async fn main() {
    println!("🚀 HighBridgeChain FULL node avviato");

    // Carica configurazione
    let cfg = NodeConfig::load_default();

    // Carica stato iniziale (genesis)
    let mut state = State::load_genesis(&cfg.genesis_path);

    // Inizializza VM
    let vm = ZLangVm::new();

    // Inizializza networking
    let mut network = Network::new(&cfg);

    // Inizializza consenso
    let mut consensus = ConsensusEngine::new(&cfg, &mut state, vm, &mut network);

    // Avvia RPC server
    tokio::spawn(async move {
        start_rpc_server("127.0.0.1:8545").await;
    });

    // Avvia consenso (loop principale)
    consensus.run().await;
}

echo "[7] Cargo build workspace"
cd "$ROOT"
cargo build

echo "[✔] HighBridgeChain fixed & built."
EOF
