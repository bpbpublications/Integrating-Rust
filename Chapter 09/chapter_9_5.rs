fn main() {
    let result: Result<i32, &str> = Ok(42);
    let value = result.unwrap();
    println!("The value is {}", value);
}
