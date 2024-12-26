use crossterm::{
    execute,
    cursor::{MoveTo, MoveToNextLine},
    style::{Color, Print, SetForegroundColor},
};
use std::io;

fn main() {
    // Basic output using println!
    println!("Hello, Rustaceans!");

    // Advanced terminal control using crossterm
    execute!(
        io::stdout(),
        MoveTo(5, 5),
        SetForegroundColor(Color::Yellow),
        Print("This is a colored message!"),
        MoveToNextLine(1),
    )
    .expect("Failed to execute crossterm commands");
}
