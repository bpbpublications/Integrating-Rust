use std::fs;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("file.txt")?;
    let modified = metadata.modified()?;
    println!("Last modified: {:?}", modified);
    Ok(())
}
