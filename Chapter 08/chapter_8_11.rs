use std::fs;

fn main() -> std::io::Result<()> {
    let path = "/path/to/directory";
    for entry in fs::read_dir(path)? {
        let dir_entry = entry?;
        let file_name = dir_entry.file_name();
        let file_type = dir_entry.file_type()?;
        println!(
            "File name: {:?}, File type: {:?}",
            file_name,
            file_type
        );
    }
    Ok(())
}
