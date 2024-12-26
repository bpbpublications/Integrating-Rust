use rhai::{Engine, EvalAltResult};

fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let mut engine = Engine::new();
    engine.register_fn("greet", greet);
}
