fn main() {
    let mut num = 10;

    // Creating a raw pointer from a mutable reference
    let raw_ptr: *mut i32 = &mut num;

    unsafe {
        // Dereferencing a raw pointer
        *raw_ptr += 10;
    }

    println!("Modified value: {}", num);
}