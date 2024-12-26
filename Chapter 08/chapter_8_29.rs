use std::{fs, os::unix::fs::MetadataExt};

fn main() {
    let metadata = fs::metadata("file.txt").unwrap();
    let gid = metadata.gid();

    let group_id = users::get_group_by_gid(gid).unwrap();
    let groupname = group_id.name().to_string_lossy();
    println!("The file is owned by group {}", groupname);
}
