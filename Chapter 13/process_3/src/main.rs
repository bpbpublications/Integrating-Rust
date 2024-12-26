use std::process::Command;

fn main() {
    let mut child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to start child process");

    let status = child.wait().expect("Failed to wait for child process");
    println!("Child process exited with: {:?}", status);
}

