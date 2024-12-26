use std::fs;

fn main() {
    let metadata = fs::metadata("file.txt").expect("Failed to get metadata");

    if metadata.is_file() {
        println!("File");
    } else if metadata.is_dir() {
        println!("Directory");
    } else if metadata.file_type().is_symlink() {
        println!("Symbolic link");
    } else {
        println!("Other");
    }
}
