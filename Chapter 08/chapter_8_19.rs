use std::os::unix::fs;

fn main() -> std::io::Result<()> {
    // Create a symbolic link to a file
    std::os::unix::fs::symlink("original.txt", "link.txt")?;

    // Create a symbolic link to a directory
    std::fs::symlink("original_directory", "link_directory")?;

    Ok(())
}
