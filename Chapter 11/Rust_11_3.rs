// Example using async-std
use async_std::fs::File;
use async_std::io::{ReadExt, Result};
use async_std::task;

async fn read_async() -> Result<String> {
    let mut file = File::open("/dev/input/event2").await?;
    let mut buffer = [0u8; 1024];

    let bytes_read = file.read(&mut buffer).await?;
    let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

    Ok(data)
}

fn main() {
    let data = task::block_on(read_async());
    match data {
        Ok(data) => println!("Read data: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
