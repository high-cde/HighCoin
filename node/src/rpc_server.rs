use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn start_rpc_server(addr: &str) {
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[RPC] Errore bind {}: {}", addr, e);
            return;
        }
    };

    println!("[RPC] Listening on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 8192];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let request = String::from_utf8_lossy(&buffer[..n]);

                    // Trova il JSON nel body
                    let json_start = match request.find("{") {
                        Some(i) => i,
                        None => {
                            send_http_error(&mut socket, "Invalid request").await;
                            return;
                        }
                    };

                    let json_str = &request[json_start..];

                    let req_json: serde_json::Value = match serde_json::from_str(json_str) {
                        Ok(v) => v,
                        Err(_) => {
                            send_http_error(&mut socket, "JSON parse error").await;
                            return;
                        }
                    };

                    let response_json = handle_rpc(req_json).await;
                    send_http_json(&mut socket, response_json).await;
                }
                _ => {}
            }
        });
    }
}

async fn send_http_json(socket: &mut tokio::net::TcpStream, json: serde_json::Value) {
    let body = json.to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = socket.write_all(response.as_bytes()).await;
}

async fn send_http_error(socket: &mut tokio::net::TcpStream, msg: &str) {
    let body = json!({
        "jsonrpc": "2.0",
        "error": { "code": -32600, "message": msg },
        "id": null
    })
    .to_string();

    let response = format!(
        "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );

    let _ = socket.write_all(response.as_bytes()).await;
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

        _ => json!({
            "jsonrpc": "2.0",
            "error": { "code": -32601, "message": "Method not found" },
            "id": id
        }),
    }
}
