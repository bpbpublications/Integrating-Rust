use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!")?;

    let mut contents = String::new();
    File::open("example.txt")?.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);
    Ok(())
}

