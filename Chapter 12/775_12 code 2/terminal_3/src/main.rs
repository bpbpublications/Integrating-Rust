use crossterm::{cursor, execute};
use std::io;

fn main() {
    // Move the cursor to row 10, column 20
    execute!(io::stdout(), cursor::MoveTo(20, 10))
        .expect("Failed to move cursor");

    println!("Cursor moved!");
}
