use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use serde_json::Value;
use uuid::Uuid;
use colored::Colorize;

use crate::helper::logs::*;

pub struct TcpServer {
    clients: Arc<Mutex<HashMap<Uuid, TcpStream>>>,
    listener: TcpListener,
}

impl TcpServer {
    fn new(addr: &str) -> std::io::Result<TcpServer> {
        let listener = TcpListener::bind(addr)?;
        log_info(&format!("TCP Server listening on {}", addr.bold().purple()));
        let clients = Arc::new(Mutex::new(HashMap::new()));
        let server = Self {
            clients: clients.clone(),
            listener: listener.try_clone()?
        };
        Ok(server)
    }

    fn handle_client(&self, mut stream: TcpStream, client_uuid: Uuid) {
        let mut buf = [0; 1024];
        loop {
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    if n == 1 {
                        // Ignore empty messages
                        continue;
                    }
                    println!("Received {} bytes from client {}", n, client_uuid);
                    // show the uuid of the client and the decoded message
                    let message = String::from_utf8_lossy(&buf[..n]);
                    println!("Client UUID: {}, Message: {}", client_uuid, message);

                    // try to convert to Value
                    let value: Result<Value, _> = serde_json::from_str(&message);
                    let json_value = match value {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Error parsing JSON: {}", e);
                            continue;
                        }
                    };

                    /* 
                    // show client uuids list
                    let clients = self.clients.lock().unwrap();
                    let client_uuids: Vec<Uuid> = clients.keys().cloned().collect();
                    println!("Connected clients: {:?}", client_uuids);*/

                    /* 
                    if message.starts_with("broadcast:") {
                        for (uuid, mut client) in clients.iter() {
                            if let Err(e) = client.write_all(&buf[..n]) {
                                eprintln!("Error writing to client {}: {}", uuid, e);
                            }
                        }
                    } else {
                        if let Err(e) = stream.write_all(&buf[..n]) {
                            eprintln!("Error writing to stream: {}", e);
                            break;
                        }
                    }*/
                }
                Ok(_) | Err(_) => {
                    // On EOF or error, remove the client from the hashmap
                    let mut clients = self.clients.lock().unwrap();
                    clients.remove(&client_uuid);
                    break;
                }
            }
        }
    }

    pub fn start(self) -> std::io::Result<()> {
        
        // start a new thread to listen for incoming connections
        thread::spawn(move || {
            for stream in self.listener.incoming() {
                let stream = stream.expect("Failed to accept connection");
                let client_uuid = Uuid::new_v4();
                log_info(&format!("New client connected with UUID: {}", client_uuid.to_string().bold().purple()));
        
                // Store the client in the hashmap
                let clients_clone = Arc::clone(&self.clients);
                let self_clone = self.clone(); // Clone the server for each thread
                let mut clients = clients_clone.lock().unwrap();
                clients.insert(client_uuid, stream.try_clone().expect("Failed to clone stream"));
        
                // unlock the clients
                drop(clients);
        
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    self_clone.handle_client(stream, client_uuid);
                });
            }
        });

        Ok(())
    }
    
    pub fn get_clients(&self) -> Vec<Uuid> {
        self.clients.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_clients_str(&self) -> Vec<String> {
        self.clients.lock().unwrap().keys().cloned().map(|uuid| uuid.to_string()).collect()
    }

    pub fn send_hello_to_client(&self, client_uuid: Uuid) {
        let clients = self.clients.lock().unwrap();
        if let Some(mut client) = clients.get(&client_uuid) {
            let message = "Hello from server";
            if let Err(e) = client.write_all(message.as_bytes()) {
                log_error(&format!("Error writing to client {}: {}", client_uuid, e));
            }
        }
    }

    pub fn send_to_client(&self, client_uuid: Uuid, message: &str) {
        let clients = self.clients.lock().unwrap();
        if let Some(mut client) = clients.get(&client_uuid) {
            if let Err(e) = client.write_all(message.as_bytes()) {
                log_error(&format!("Error writing to client {}: {}", client_uuid, e));
            }
        }
    }

    pub fn send_debug_message(&self, client_uuid: Uuid) -> bool {
        let clients = self.clients.lock().unwrap();
        if let Some(mut client) = clients.get(&client_uuid) {
            let message = "Hello from server";
            if let Err(e) = client.write_all(message.as_bytes()) {
                log_error(&format!("Error writing to client {}: {}", client_uuid, e));
            }
            return true;
        }
        false
    }

    pub fn send_message(&self, client_uuid: Uuid, data: String) -> bool {
        let clients = self.clients.lock().unwrap();
        if let Some(mut client) = clients.get(&client_uuid) {
            let message = data;
            if let Err(e) = client.write_all(message.as_bytes()) {
                log_error(&format!("Error writing to client {}: {}", client_uuid, e));
            }
            return true;
        }
        false
    }

}


impl Clone for TcpServer {
    fn clone(&self) -> Self {
        Self {
            clients: self.clients.clone(),
            listener: self.listener.try_clone().expect("Failed to clone listener")
        }
    }
}


pub fn create_server() -> std::io::Result<TcpServer> {
    TcpServer::new("0.0.0.0:15002")
}