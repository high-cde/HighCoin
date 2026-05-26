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
