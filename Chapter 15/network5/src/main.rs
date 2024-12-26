use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Create a TCP listener bound to address 127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Async TCP server listening on 127.0.0.1:8080...");

    // Accept and handle incoming connections asynchronously
    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            // Read data from the client
            let mut buffer = [0; 1024];
            if let Ok(size) = stream.read(&mut buffer).await {
                // Handle the received data
                let received_data = &buffer[..size];
                let received_str = String::from_utf8_lossy(received_data);
                println!(
                    "Received {} bytes from {}: {}",
                    size,
                    stream.peer_addr().unwrap(),
                    received_str
                );

                // Send a response back to the client
                if let Err(e) = stream.write_all(b"Hello from async TCP server!").await {
                    eprintln!("Error sending response: {}", e);
                }
            }
        });
    }

    Ok(())
}
