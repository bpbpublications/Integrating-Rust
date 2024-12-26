use std::path::PathBuf;

fn main() {
    let base_path = PathBuf::from("/home/user");
    let new_path = base_path.join("documents");

    println!("{:?}", new_path); // prints "/home/user/documents"
}
