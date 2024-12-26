use rhai::{Engine, Scope};

fn main() {
    let engine = Engine::new();
    let mut scope = Scope::new();

    let name = "John";
    scope.push("name", name);
    engine
        .eval_file_with_scope::<()>(&mut scope, "/path/to/greet.rhai".into())
        .unwrap();
}
