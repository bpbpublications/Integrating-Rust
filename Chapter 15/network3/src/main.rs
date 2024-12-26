use std::net::UdpSocket;

fn main() {
    // Create a UDP socket bound to address 127.0.0.1:8888
    let socket = UdpSocket::bind("127.0.0.1:8888").expect("Failed to bind");

    println!("UDP server listening on 127.0.0.1:8888...");

    // Receive data from clients
    loop {
        let mut buffer = [0; 1024];
        let (size, source) = socket.recv_from(&mut buffer).expect("Failed to receive data");

        // Handle the received data
        let received_data = &buffer[..size];
        let received_str = String::from_utf8_lossy(received_data);
        println!("Received {} bytes from {}: {}", size, source, received_str);

        // Send a response back to the client
        socket
            .send_to(b"Hello from UDP server!", &source)
            .expect("Failed to send response");
    }
}
