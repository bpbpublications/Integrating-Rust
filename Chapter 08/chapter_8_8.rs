use std::fs;

fn main() {
    match fs::remove_dir_all("mydir") {
        Ok(()) => println!("Directory removed successfully"),
        Err(e) => println!("Error removing directory: {}", e),
    }
}
