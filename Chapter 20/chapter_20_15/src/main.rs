use rhai::Engine;
use std::fs;

fn main() {
    let engine = Engine::new();

    // Read the script file contents
    let script = fs::read_to_string("invalid_script.rhai").expect("Unable to read script file");

    // Evaluate the script
    match engine.eval::<()>(&script) {
        Ok(_) => println!("Script executed successfully"),
        Err(err) => println!("Error: {}", err),
    }
}
