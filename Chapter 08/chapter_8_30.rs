use std::fs;

fn main() -> std::io::Result<()> {
    let file = fs::File::open("example.txt")?;
    let metadata = file.metadata()?;
    let size = metadata.len();
    println!("Size of file: {} bytes", size);
    Ok(())
}
