use std::fs;

fn main() -> std::io::Result<()> {
    let path = "tmp/my/nested/directory";

    // Create the directory and its parent directories recursively
    fs::create_dir_all(path)?;

    Ok(())
}
