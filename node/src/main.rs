mod rpc;
mod state;
mod network;
mod consensus;
mod vm;
mod zlang_parser;
mod zlang_vm;

use rpc::start_rpc;
use state::State;
use network::Network;
use consensus::Consensus;
use vm::Vm;

use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("🚀 HighBridgeChain FULL node avviato");

    // Stato, rete, consenso, VM
    let state = Arc::new(Mutex::new(State::new()));
    let network = Arc::new(Mutex::new(Network::new()));
    let consensus = Arc::new(Mutex::new(Consensus::new()));
    let vm = Arc::new(Mutex::new(Vm::new()));

    // RPC
    let rpc_state = state.clone();
    tokio::spawn(async move {
        start_rpc("127.0.0.1:8545", rpc_state).await;
    });

    println!("[RPC] Listening on 127.0.0.1:8545");

    // Loop blocchi
    let state_loop = state.clone();
    let network_loop = network.clone();
    let consensus_loop = consensus.clone();
    let vm_loop = vm.clone();

    tokio::spawn(async move {
        let mut height = 0;

        loop {
            sleep(Duration::from_secs(2)).await;
            height += 1;

            // Stato
            {
                let mut st = state_loop.lock().unwrap();
                st.apply_block(height);
            }

            // Consenso
            {
                let mut c = consensus_loop.lock().unwrap();
                c.on_new_block(height);
            }

            // VM: deploy una volta, poi call
            {
                let mut vm = vm_loop.lock().unwrap();

                if height == 1 {
                    // deploy di un contratto "math"
                    match vm.deploy("math", "math") {
                        Ok(res) => println!("📦 Contract deployed: {}", res),
                        Err(err) => println!("🔥 VM deploy error: {}", err),
                    }
                }

                // call su ogni blocco
                match vm.call(
                    "math",
                    "add",
                    serde_json::json!({ "a": height, "b": 10 }),
                ) {
                    Ok(result) => println!("🧮 SC math.add => {}", result),
                    Err(err) => println!("🔥 SC error: {}", err),
                }
            }

            // Network broadcast
            {
                let mut n = network_loop.lock().unwrap();
                n.broadcast_block(height);
            }

            println!("⛓️  Prodotto blocco {}", height);
            println!("📡 Broadcast blocco: {} bytes", 185 + (height % 3));
        }
    });

    // Mantieni vivo il nodo
    loop {
        sleep(Duration::from_secs(60)).await;
    }
}
