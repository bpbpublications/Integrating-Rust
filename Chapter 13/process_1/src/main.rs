use std::process::Command;

fn main() {
    let output = Command::new("dir")
        .arg("-l")
        .output()
        .expect("Failed to execute process");

    let text_output = String::from_utf8_lossy(&output.stdout);
    println!("Output:\n{}", text_output);
}
