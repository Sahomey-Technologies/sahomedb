use serde_json::{from_str, to_string};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

use super::ErrorResponse;

// Set some type aliases for convenience.
type RequestBody = HashMap<String, String>;
type KeyValue = Arc<Mutex<HashMap<String, String>>>;

pub struct Server {
    addr: SocketAddr,
    kv: KeyValue,
}

impl Server {
    pub async fn new(host: &str, port: &str) -> Server {
        let addr = format!("{}:{}", host, port).parse().unwrap();
        let kv = Arc::new(Mutex::new(HashMap::new()));
        Server { addr, kv }
    }

    pub async fn serve(&self) {
        // Bind a listner to the socket address
        let listner = TcpListener::bind(self.addr).await.unwrap();

        // Accept and handle connections from clients.
        loop {
            let (stream, _) = listner.accept().await.unwrap();
            let handler = self.handle_connection(stream).await;
            spawn(async move { handler });
        }
    }
    async fn handle_connection(&self, mut stream: TcpStream) {
        loop {
            // Read data from client
            let data = self._read(&mut stream).await;

            // when the client disconnects
            if data.is_empty() {
                break;
            }

            let json: RequestBody = from_str(&data).unwrap();
            if json.get("command") == None {
                let _code = "command_required";
                let _msg = "The command key is missing.";
                let _err = ErrorResponse::new(_code, _msg).response();
                self._write(&mut stream, &_err).await;
                continue;
            }
            // Normalize the command to lowercase.
            let command = json.get("command").unwrap().to_lowercase();

            let error = {
                let code = "unrecognized_command";
                let msg = format!("Unrecognized command: {}.", command);

                ErrorResponse::new(code, &msg).response()
            };

            // Handle the command for the response
            let response = match command.as_str() {
                "set" => self.handle_set(json).await,
                "get" => self.handle_get(json).await,
                _ => error,
            };

            // Write the data back to the client.
            self._write(&mut stream, &response).await;
        }
    }

    async fn handle_set(&self, json: RequestBody) -> String {
        // Validate that the key and value are present.
        if json.get("key") == None || json.get("value") == None {
            let _code = "key_value_required";
            let _msg = "The key or value is missing.";
            return ErrorResponse::new(_code, _msg).response();
        }

        // Get the key and value from the request body.
        let key = json.get("key").unwrap().to_string();
        let value = json.get("value").unwrap().to_string();
        // Create a map for the response.
        let mut map = HashMap::new();
        map.insert(key.clone(), value.clone());
        // Insert the key and value into the database.
        self.kv.lock().unwrap().insert(key, value);
        to_string(&map).unwrap()
    }

    async fn handle_get(&self, json: RequestBody) -> String {
        // Validate that the key is present.
        if json.get("key") == None {
            let _code = "key_required";
            let _msg = "The key is missing.";
            return ErrorResponse::new(_code, _msg).response();
        }

        let key = json.get("key").unwrap().to_string();

        // Get the value from the database.
        let kv = self.kv.lock().unwrap();
        let value = kv.get(&key);

        // Error handling when value not found.
        if value.is_none() {
            let _code = "value_not_found";
            let _msg = "No value set for this key";
            return ErrorResponse::new(_code, _msg).response();
        }

        // Create a map for the response.
        let mut map = HashMap::new();
        map.insert(key, value.unwrap().to_string());

        to_string(&map).unwrap()
    }

    async fn _read(&self, stream: &mut TcpStream) -> String {
        let mut buf = vec![0; 1024];
        let n = match stream.read(&mut buf).await {
            Ok(n) => n,
            Err(e) => {
                println!("ReadError: {}", e);
                return String::new();
            }
        };

        String::from_utf8_lossy(&buf[0..n]).to_string()
    }

    async fn _write(&self, stream: &mut TcpStream, data: &str) {
        match stream.write_all(data.as_bytes()).await {
            Ok(_) => (),
            Err(e) => println!("WriteError: {}", e),
        }
    }
}
