use std::fs;

fn main() {
    if let Err(_) = fs::metadata("file.txt") {
        println!("File does not exist");
    } else {
        println!("File exists");
    }
}
