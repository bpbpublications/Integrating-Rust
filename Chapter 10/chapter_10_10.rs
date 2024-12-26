use std::sync::Arc;

fn main() {
    let shared_data = Arc::new(vec![1, 2, 3]);
    let threads: Vec<_> = (0..3).map(|_| {
        let shared_data = Arc::clone(&shared_data);
        std::thread::spawn(move || {
            println!("Shared data: {:?}", shared_data);
        })
    }).collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
