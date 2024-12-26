use std::fs::canonicalize;

fn compare_paths() {
    let path1 = "/path/to/file";
    let path2 = "/path/./to/../to/file";
    
    // Canonicalize the paths to resolve them to their absolute paths
    let abs_path1 = canonicalize(path1).unwrap();
    let abs_path2 = canonicalize(path2).unwrap();
    
    // Compare the absolute paths
    if abs_path1 == abs_path2 {
        println!("Paths refer to the same file/directory");
    } else {
        println!("Paths refer to different files/directories");
    }
}
