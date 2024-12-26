use rhai::{Engine, EvalAltResult};

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    engine.register_fn("add", add);

    // Execute Rhai script calling the Rust function
    let result = engine.eval::<i64>("add(10, 20)")?;
    println!("Result: {}", result);

    Ok(())
}
