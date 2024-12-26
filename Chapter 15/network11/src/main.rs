use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    // Send a message to the server
    socket
        .send_to(b"Hello from UDP client!", "127.0.0.1:8888")?;

    // Receive the response from the server
    let mut buffer = [0; 1024];
    let (size, _) = socket.recv_from(&mut buffer)?;

    // Print the server's response
    println!(
        "Server response: {:?}",
        String::from_utf8_lossy(&buffer[..size])
    );

    Ok(())
}
