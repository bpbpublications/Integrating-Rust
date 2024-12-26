use std::thread;

fn main() {
    // Create a new thread and spawn it
    let handle = thread::spawn(|| {
        // Code to be executed in the new thread
        println!("Hello from a new thread!");
    });

    // Wait for the new thread to finish executing
    handle.join().expect("Failed to join the thread.");
}
