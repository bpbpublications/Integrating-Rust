use std::process::{Command, Stdio};

fn main() {
    let mut cmd = Command::new("echo");
    cmd.arg("Hello, Rust!")
        .stdout(Stdio::piped());

    let child = cmd.spawn().expect("Failed to start child process");

    let output = child.wait_with_output().expect("Failed to wait for child process");
    println!("Child process exited with: {:?}", output.status);
    println!("Child process output: {}", String::from_utf8_lossy(&output.stdout));
}

