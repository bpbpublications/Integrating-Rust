use crossterm::{
    event::{Event, MouseEvent, MouseButton, MouseEventKind},
    terminal,
};

fn main() {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let event = crossterm::event::read().expect("Failed to read event");

    terminal::disable_raw_mode().expect("Failed to disable raw mode");

    match event {
        Event::Mouse(MouseEvent {
            kind,
            row,
            column,
            .. /*modifiers,*/
        }) => {
            match kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    println!("Left mouse button clicked at row {}, column {}", row, column);
                }
                _ => println!("Other mouse event"),
            }
        }
        _ => println!("Other event"),
    }
}
