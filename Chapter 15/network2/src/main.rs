use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // Connect to the server at 127.0.0.1:8080
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    // Send data to the server
    stream.write_all(b"Hello from client!")?;

    // Read the server's response
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    // Trim the buffer to remove trailing null bytes
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Print the trimmed response
    println!("Server response: {:?}", response);

    Ok(())
}
