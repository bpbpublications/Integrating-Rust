use std::fs::File;

fn main() -> std::io::Result<()> {
    let _file = File::create("example.txt")?;
    Ok(())
}
