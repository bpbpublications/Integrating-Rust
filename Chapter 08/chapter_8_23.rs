use std::path::Path;

fn main() {
    let path = Path::new("file.txt");
    if path.exists() {
        println!("File exists");
    } else {
        println!("File does not exist");
    }
}
