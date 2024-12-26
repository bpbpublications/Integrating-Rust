use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server at 127.0.0.1:8080
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Send a message to the server
    stream.write_all(b"Hello from async TCP client!").await?;

    // Read the server's response
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).await?;

    // Print the server's response
    println!(
        "Server response: {:?}",
        String::from_utf8_lossy(&buffer[..size])
    );

    Ok(())
}
