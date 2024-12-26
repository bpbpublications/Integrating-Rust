fn main() {
    let my_string = String::from("Hello, world!");
    let slice1 = &my_string[..5]; // "Hello"
    let slice2 = &my_string[7..]; // "world!"

    println!("{}, {}", slice1, slice2);
}
