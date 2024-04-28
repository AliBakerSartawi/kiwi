use lib::{
    executer::handle_command,
    store::{ConcurrentStore, Store},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let store = Store::new();

    let host = "127.0.0.1";
    let port = "6131";
    let addr = format!("{host}:{port}");
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to port {port}\nError: {e}"));
    println!("Listening on {}", addr);

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .unwrap_or_else(|e| panic!("Failed to accept connection\nError: {e}"));
        let store = store.clone();

        tokio::spawn(async {
            handle_connection(socket, store).await;
        });
    }

}

async fn handle_connection(socket: TcpStream, store: ConcurrentStore) {
    let (mut reader, mut writer) = tokio::io::split(socket);

    loop {
        let mut buf = vec![0; 1024];
        let n = match reader.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        };

        let input = String::from_utf8_lossy(&buf[..n]).to_string();
        let response = match handle_command(input, store.clone()).await {
            Ok(response) => response,
            Err(e) => e,
        } + "\n\n";

        if let Err(e) = writer.write_all(response.as_bytes()).await {
            eprintln!("Failed to write to socket: {}", e);
            return;
        }
    }
}
