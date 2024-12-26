use std::fs;

fn main() -> std::io::Result<()> {
    // Create a directory
    fs::create_dir("mydir")?;

    // Move the directory to a different location
    fs::rename("mydir", "/tmp/mydir")?;

    Ok(())
}
