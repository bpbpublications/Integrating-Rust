use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread to send messages
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "another", "thread"];

        for msg in messages {
            sender.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // Receive messages in the main thread
    for received in receiver {
        println!("Received: {}", received);
    }
}
