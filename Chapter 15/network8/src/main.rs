use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // Read data from the client
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).expect("Failed to read from client");

    // Handle the received data
    let received_data = &buffer[..size];
    let received_str = String::from_utf8_lossy(received_data);
    println!("Received {} bytes from {}: {}", size, stream.peer_addr().unwrap(), received_str);

    // Send a response back to the client
    stream.write_all(b"Hello from TCP server!").expect("Failed to write to client");
}

fn main() {
    // Create a TCP listener bound to address 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    println!("TCP server listening on 127.0.0.1:8080...");

    // Accept and handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the connection
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
