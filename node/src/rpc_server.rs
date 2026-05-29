use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;

pub async fn start_rpc_server(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("[RPC] Listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 4096];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let request = String::from_utf8_lossy(&buffer[..n]);

                    // Estrai il JSON dal body HTTP
                    let json_start = request.find("{").unwrap_or(0);
                    let json_str = &request[json_start..];

                    let response_json = match serde_json::from_str::<serde_json::Value>(json_str) {
                        Ok(req) => handle_rpc(req).await,
                        Err(_) => json!({
                            "jsonrpc": "2.0",
                            "error": { "code": -32700, "message": "Parse error" },
                            "id": null
                        }),
                    };

                    let response_body = response_json.to_string();

                    let http_response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        response_body.len(),
                        response_body
                    );

                    let _ = socket.write_all(http_response.as_bytes()).await;
                }
                _ => {}
            }
        });
    }
}

async fn handle_rpc(req: serde_json::Value) -> serde_json::Value {
    let method = req["method"].as_str().unwrap_or("");
    let id = req["id"].clone();

    match method {
        "ping" => json!({
            "jsonrpc": "2.0",
            "result": "pong",
            "id": id
        }),

        "getinfo" => json!({
            "jsonrpc": "2.0",
            "result": {
                "name": "HighCoin",
                "version": "0.1",
                "protocol": 1
            },
            "id": id
        }),

        "getheight" => json!({
            "jsonrpc": "2.0",
            "result": 0,
            "id": id
        }),

        _ => json!({
            "jsonrpc": "2.0",
            "error": { "code": -32601, "message": "Method not found" },
            "id": id
        }),
    }
}
