// Declaration of an unsafe function
unsafe fn unsafe_function() {
    // Unsafe function body
    println!("Executing unsafe function");
}

fn main() {
    // Calling the unsafe function within an unsafe block
    unsafe {
        unsafe_function();
    }
}
