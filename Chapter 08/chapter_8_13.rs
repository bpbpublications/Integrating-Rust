use std::path::Path;

fn main() {
    let path1 = Path::new("/home/user");
    let path2 = Path::new("Documents");
    let new_path = path1.join(path2);
    println!("New Path: {:?}", new_path);
}
