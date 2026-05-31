use node::{rpc::start_rpc, state::State};
use std::sync::{LazyLock, Mutex};

static STATE: LazyLock<State> = LazyLock::new(|| State {
    height: 0,
    vm: Mutex::new(node::vm::Vm::new()),
});

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        start_rpc(&STATE, "0.0.0.0:8765").await;
    });

    println!("🚀 HighCoin Node avviato");
    println!("📡 RPC su http://127.0.0.1:8765");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
