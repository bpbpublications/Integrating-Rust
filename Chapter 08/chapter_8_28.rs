use std::{fs, os::unix::fs::MetadataExt};

fn main() {
    let metadata = fs::metadata("file.txt").unwrap();
    let uid = metadata.uid();

    let username = users::get_user_by_uid(uid)
        .unwrap()
        .name()
        .to_string_lossy();
    println!("The file is owned by user {}", username);
}
