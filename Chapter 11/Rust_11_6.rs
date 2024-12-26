use std::io;

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    match read_input() {
        Ok(name) => println!("Hello, {}!", name),
        Err(err) => eprintln!("Error: {}", err),
    }
}
