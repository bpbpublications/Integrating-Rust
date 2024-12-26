use std::path::Path;
use sys_info::{disk_info, DiskInfo};

fn get_disk_space_info<P: AsRef<Path>>(_path: P) -> Result<DiskInfo, sys_info::Error> {
    disk_info()
}

fn main() {
    let path = "/"; // Replace with the path you want to query
    match get_disk_space_info(path) {
        Ok(disk) => {
            println!("Total space: {} MB", disk.total / 1024);
            println!("Free space: {} MB", disk.free / 1024);
        }
        Err(e) => eprintln!("Failed to retrieve disk info: {}", e),
    }
}
