use std::io;

fn main() {
    let mut input = String::new();

    println!("Please enter your name: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Hello, {}!", input.trim());
}

