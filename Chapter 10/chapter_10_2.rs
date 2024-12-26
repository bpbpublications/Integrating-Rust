use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared mutable data protected by a mutex
    let counter = Arc::new(Mutex::new(0));

    // Spawn multiple threads to increment the counter
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                // Acquire the mutex lock
                let mut num = counter.lock().unwrap();

                // Modify the shared data
                *num += 1;
            })
        })
        .collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Counter: {}", *counter.lock().unwrap());
}
