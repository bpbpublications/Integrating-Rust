use std::fs;

fn main() {
    let metadata = fs::metadata("file.txt").expect("Failed to get metadata");

    println!("File size: {} bytes", metadata.len());
    println!("Created: {:?}", metadata.created());
    println!("Modified: {:?}", metadata.modified());
    println!("Permissions: {:?}", metadata.permissions());
}
