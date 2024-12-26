use std::process::{Command, Stdio};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut cmd = Command::new("echo")
        .arg("Hello, Rust!")
        .stdout(Stdio::piped())
        .spawn()?;

    let mut output = String::new();
    cmd.stdout.take().unwrap().read_to_string(&mut output)?;

    println!("Child process output: {}", output);
    Ok(())
}

