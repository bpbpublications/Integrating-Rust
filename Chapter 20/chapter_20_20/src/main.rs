use rhai::Engine;

#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut engine = Engine::new();
    engine.register_type::<Person>();
}
