use std::fs;

fn main() -> std::io::Result<()> {
    // Create a symbolic link to a file
    fs::symlink("original.txt", "link.txt")?;
    
    // Follow the symbolic link and print the path it points to
    let path = fs::read_link("link.txt")?;
    println!("Link points to: {}", path.display());
    
    Ok(())
}
