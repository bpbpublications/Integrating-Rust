use std::thread;
use std::time;

fn main() {
    let handle = thread::spawn(|| {
        // Perform some work in the thread
        println!("Thread started");
        for i in 1..=5 {
            println!("Count: {}", i);
            thread::sleep(time::Duration::from_millis(1000));
        }
        println!("Thread finished");
    });

    // Wait for the thread to finish
    handle.join().unwrap();

    println!("Main thread finished");
}
