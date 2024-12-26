use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let result = engine.eval::<i64>("40 + 2")?;
    println!("Result: {}", result);

    Ok(())
}
