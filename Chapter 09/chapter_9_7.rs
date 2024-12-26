fn main() {
    let option: Option<&str> = None;
    let value = option.unwrap();
    println!("Value: {}", value);
}
