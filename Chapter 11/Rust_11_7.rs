use std::io::{self, Write};

fn print_greeting(name: &str) -> io::Result<()> {
    let message = format!("Hello, {}!", name);
    io::stdout().write_all(message.as_bytes())?;
    Ok(())
}

fn main() {
    let name = "Abhishek";
    match print_greeting(name) {
        Ok(()) => println!(),
        Err(err) => eprintln!("Error: {}", err),
    }
}
