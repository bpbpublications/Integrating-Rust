use std::io;

fn main() {
    println!("Please enter your name:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("Hello, {}!", input.trim());
}
