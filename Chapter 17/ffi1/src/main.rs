// File: main.rs

// Declaration of the external C function
extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
}

fn main() {
    // Calling the external function
    let result = unsafe {
        add_numbers(5, 6)
    };

    // Printing the result
    println!("Result of adding numbers: {}", result);
}
