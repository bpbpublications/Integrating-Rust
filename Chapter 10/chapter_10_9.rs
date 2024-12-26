fn main() {
    let data = "Hello, World!".to_string();
    let thread = std::thread::spawn(move || {
        println!("Data received: {}", data);
    });
    thread.join().unwrap();
}
