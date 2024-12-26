use crossterm::{
    event::KeyEvent, terminal,
};

fn main() {
    //terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let key_event = crossterm::event::read().expect("Failed to read event");

    //terminal::disable_raw_mode().expect("Failed to disable raw mode");

    match key_event {
        crossterm::event::Event::Key(KeyEvent {
            code,
            modifiers,
            state,
            ..
        }) => {
            println!("Key pressed: {:?}", code);
            println!("Modifiers: {:?}", modifiers);
            println!("State: {:?}", state);
        }
        _ => println!("Other event"),
    }
}
