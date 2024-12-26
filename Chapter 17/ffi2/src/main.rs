// File: main.rs

// Declaration of the external C function
extern "C" {
    fn modify_values(x: *mut i32, y: *mut i32);
}

fn main() {
    // Initialize two integers
    let mut a = 5;
    let mut b = 7;

    // Display the initial values
    println!("Before modification: a = {}, b = {}", a, b);

    // Call the external function to modify the values through pointers
    unsafe {
        modify_values(&mut a, &mut b);
    }

    // Display the modified values
    println!("After modification: a = {}, b = {}", a, b);
}
