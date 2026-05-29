use crate::state::State;
use crate::rpc_server::start_rpc_server;

pub async fn start_rpc(_state: &'static State, addr: &str) {
    println!("[NODE] Avvio RPC su {}", addr);
    start_rpc_server(addr).await;
}
