use std::thread;

fn main() {
    let mut counter = 0;

    let handle1 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            counter += 1;
        }
    });

    let handle2 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            counter += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Counter: {}", counter);
}
