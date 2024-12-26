use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("/home/user/dir/../file.txt");
    let canonicalized = path.canonicalize().unwrap();
    println!("{:?}", canonicalized);
}
