// Declaration of a mutable static variable
static mut COUNTER: i32 = 0;

fn main() {
    unsafe {
        // Accessing and modifying the mutable static variable
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
