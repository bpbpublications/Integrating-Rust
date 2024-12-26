use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handle1 = thread::spawn({
        let counter = Arc::clone(&counter);
        move || {
            for _ in 0..1_000_000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        }
    });

    let handle2 = thread::spawn({
        let counter = Arc::clone(&counter);
        move || {
            for _ in 0..1_000_000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Counter: {}", *counter.lock().unwrap());
}
