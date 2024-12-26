use std::fs;

fn main() -> std::io::Result<()> {
    // Create a directory
    fs::create_dir("mydir")?;

    // Rename the directory
    fs::rename("mydir", "newdir")?;

    Ok(())
}
