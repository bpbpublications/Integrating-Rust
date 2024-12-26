use std::fs::File;
use std::io::{Read, Result};

fn read_from_device() -> Result<String> {
    let mut file = File::open("/dev/input/event0")?; // Replace 'event0' with the appropriate device file
    let mut buffer = [0u8; 1024]; // Create a buffer to hold the read data

    let bytes_read = file.read(&mut buffer)?; // Read data from the device into the buffer

    // Convert the read bytes into a string
    let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    Ok(data)
}
