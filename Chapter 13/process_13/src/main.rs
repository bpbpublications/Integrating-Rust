use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue; // Ignore empty input
        }

        if input == "exit" {
            break; // Exit the shell
        }

        execute_command(input);
    }
}

fn execute_command(command: &str) {
    let mut parts = command.split_whitespace();
    let cmd = parts.next().expect("No command specified");

    let output = Command::new(cmd)
        .args(parts)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error: Command failed with code {:?}", output.status.code());
    }
}

