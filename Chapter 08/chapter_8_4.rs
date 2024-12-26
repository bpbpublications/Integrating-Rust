use std::fs::metadata;

fn main() -> std::io::Result<()> {
    let metadata = metadata("example.txt")?;
    let file_size = metadata.len();
    println!("File size: {} bytes", file_size);
    Ok(())
}
