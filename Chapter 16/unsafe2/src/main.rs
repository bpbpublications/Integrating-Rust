fn main() {
    let mut num = 10;

    // Creating a mutable reference
    let reference: &mut i32 = &mut num;

    // Creating a raw pointer from a mutable reference
    let raw_ptr: *mut i32 = reference;

    unsafe {
        // Dereferencing and modifying the value through the raw pointer
        *raw_ptr += 5;
    }

    // Accessing the modified value through the original reference
    println!("Modified value: {}", num);
}
