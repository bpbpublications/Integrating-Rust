use std::net::{TcpListener};
use std::io::{Read, Write};

fn main() {
    // Create a TCP listener bound to address 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    println!("Server listening on 127.0.0.1:8080...");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
                // Handle the connection
                println!("Accepted connection from: {:?}", tcp_stream.peer_addr());

                // Read and write data to the client
                let mut buffer = [0; 1024];
                tcp_stream.read(&mut buffer).expect("Failed to read from socket");
                tcp_stream.write_all(b"Hello from server!").expect("Failed to write to socket");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
