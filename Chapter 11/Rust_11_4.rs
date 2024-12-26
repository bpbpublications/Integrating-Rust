use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file_using_bufreader(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_file_using_bufreader("input.txt") {
        eprintln!("Error: {}", err);
    }
}
