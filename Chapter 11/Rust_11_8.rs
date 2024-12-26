use std::io::{self, Write};

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    print!("Enter your name: ");
    io::stdout().flush()?; // Flush stdout to ensure prompt is displayed
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn print_greeting(name: &str) -> io::Result<()> {
    let message = format!("Hello, {}!", name);
    io::stdout().write_all(message.as_bytes())?;
    Ok(())
}

fn main() {
    match read_input() {
        Ok(name) => {
            println!(); // Move to the next line after reading input
            match print_greeting(&name) {
                Ok(()) => println!(),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
