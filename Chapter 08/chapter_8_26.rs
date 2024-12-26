use std::os::unix::fs::PermissionsExt;

fn permission_queries() {
    let metadata = std::fs::metadata("/path/to/file").unwrap();

    // Check if the current user can read the file
    let is_readable = metadata.permissions().readonly();

    // Check if the current user can write to the file
    let is_writable = metadata.permissions().readonly();

    // Check if the file is executable
    let is_executable = metadata.permissions().mode() & 0o100 != 0;

    println!("Readable: {}", is_readable);
    println!("Writable: {}", is_writable);
    println!("Executable: {}", is_executable);
}
