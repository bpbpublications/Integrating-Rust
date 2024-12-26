use rhai::{Engine, Scope};

fn main() {
    let engine = Engine::new();
    let mut scope = Scope::new();

    let x = 42;
    scope.push("x", x);

    engine
        .eval_with_scope::<()>(&mut scope, "print(x);")
        .unwrap();
}
