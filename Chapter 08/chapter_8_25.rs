use std::fs;

fn main() {
    let path = "file.txt";

    if fs::metadata(path)
        .map(|m| m.is_file())
        .unwrap_or(false)
    {
        println!("File");
    } else if fs::metadata(path)
        .map(|m| m.is_dir())
        .unwrap_or(false)
    {
        println!("Directory");
    } else if fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
    {
        println!("Symbolic link");
    } else {
        println!("Other");
    }
}
