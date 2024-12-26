use std::fs;

fn read_file_contents(filename: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

fn main() {
    let filename = "data.txt";

    match read_file_contents(filename) {
        Ok(content) => println!("File Contents: {}", content),
        Err(err) => eprintln!("Error: {}", err),
    }
}
