use lib::{
    parser::{parse_input, Command},
    store::{ArcMutexStore, Store},
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

async fn handle_connection(socket: TcpStream, store: ArcMutexStore) {
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

async fn handle_command(input: String, store: ArcMutexStore) -> Result<String, String> {
    match parse_input(input)? {
        Command::Set(key, value) => {
            store.lock().await.set(key, value);
            Ok("OK".to_string())
        }
        Command::Get(key) => match store.lock().await.get(&key) {
            Some(value) => Ok(value.to_string()),
            None => Ok("Key not found".to_string()),
        },
        Command::Del(key) => match store.lock().await.del(&key) {
            Some(value) => Ok(value.to_string()),
            None => Ok("Key not found".to_string()),
        },
        Command::Help => Ok(HELP_MESSAGE.to_string()),
        Command::Unknown(cmd) => Ok(format!("Unknown command: {}", cmd)),
        Command::Empty => Ok("".to_string()),
    }
}

const HELP_MESSAGE: &str = "\
Commands:
  set <type>-<key> <value> - Set a key-value pair
       |
       └─ <type> can be one of: str, int, float, bool
          For example: `set str-name Kiwi`, `set int-age 30`, `set float-pi 3.14`, `set bool-are_kiwis_good true`
          Not providing a type will default to `str`
  get <key>                - Get the value associated with a key
  del <key>                - Delete a key-value pair
  exit                     - Exit the shell
  help                     - Show this help message";
