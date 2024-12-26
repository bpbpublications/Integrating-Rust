use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared counter variable
    let counter = Arc::new(Mutex::new(0));

    // Spawn multiple threads to increment the counter
    let mut handles = vec![];
    for _ in 0..5 {
        let counter_ref = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Acquire the lock on the counter
            let mut counter = counter_ref.lock().unwrap();

            // Increment the counter
            *counter += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    let counter = counter.lock().unwrap();
    println!("Final counter value: {}", *counter);
}
