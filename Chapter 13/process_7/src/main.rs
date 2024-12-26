use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "This is a line of text").expect("Failed to write to stdout");
}

