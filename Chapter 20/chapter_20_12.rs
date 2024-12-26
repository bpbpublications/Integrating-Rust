use rhai::{Engine, EvalAltResult};

fn main() {
    let engine = Engine::new();

    match engine.eval::<i64>("invalid syntax") {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
