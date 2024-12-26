use std::fs;

fn main() -> std::io::Result<()> {
    fs::hard_link("original_file.txt", "hard_link.txt")?;
    Ok(())
}
