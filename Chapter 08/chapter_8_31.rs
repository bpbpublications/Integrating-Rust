use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut total_size = 0;
    let path = Path::new(".");
    let dir_entries = fs::read_dir(path)?;

    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively calculate the size of subdirectories
            let size = get_dir_size(&path)?;
            total_size += size;
        } else {
            // Add the size of files to the total
            let metadata = entry.metadata()?;
            let size = metadata.len();
            total_size += size;
        }
    }

    println!("Total size: {} bytes", total_size);
    Ok(())
}

fn get_dir_size(path: &Path) -> std::io::Result<u64> {
    let mut total_size = 0;
    let dir_entries = fs::read_dir(path)?;

    for entry in dir_entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively calculate the size of subdirectories
            let size = get_dir_size(&path)?;
            total_size += size;
        } else {
            // Add the size of files to the total
            let metadata = entry.metadata()?;
            let size = metadata.len();
            total_size += size;
        }
    }

    Ok(total_size)
}
