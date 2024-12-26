fn main() {
    let maybe_name: Option<&str> = Some("Shreyansh");
    let name = maybe_name.unwrap();
    println!("Hello, {}!", name);
}
