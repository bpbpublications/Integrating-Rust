use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Spawn two threads, each sending a message
    for i in 0..2 {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone.send(i).unwrap();
        });
    }

    // Receive the messages in the main thread
    for _ in 0..2 {
        let received_message = receiver.recv().unwrap();
        println!("Received: {}", received_message);
    }
}
