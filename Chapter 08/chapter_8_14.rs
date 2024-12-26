use std::path::PathBuf;

fn main() {
    let path_string = "/home/user/documents/file.txt";
    let path_buf = PathBuf::from(path_string);
    println!("Parsed path: {:?}", path_buf);
}
