use std::net::ToSocketAddrs;

fn main() {
    // Hostname to be resolved
    let hostname = "www.google.com";

    // Perform DNS resolution for the given hostname
    match (hostname, 80).to_socket_addrs() {
        Ok(socket_addrs) => {
            for socket_addr in socket_addrs {
                println!("Resolved address: {}", socket_addr);
            }
        }
        Err(e) => {
            eprintln!("Failed to perform DNS resolution: {}", e);
        }
    }
}
