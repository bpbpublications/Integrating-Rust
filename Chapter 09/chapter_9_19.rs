fn main() {
    let number = 5;

    match number {
        0 => println!("zero"),
        1 => println!("one"),
        2..=9 => println!("between two and nine"),
        _ => println!("something else"),
    }
}
