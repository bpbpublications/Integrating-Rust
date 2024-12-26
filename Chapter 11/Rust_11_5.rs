use std::fs::File;
use std::io::{self, BufWriter, Write};

fn write_to_file_using_bufwriter(filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for i in 1..=10 {
        writeln!(writer, "Line {}", i)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = write_to_file_using_bufwriter("output.txt") {
        eprintln!("Error: {}", err);
    }
}
