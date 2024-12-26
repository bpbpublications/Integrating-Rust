use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    // Call the Rhai function 'square' from Rust
    let mut scope = rhai::Scope::new();
    let ast = engine.compile_file("/path/to/square.rhai".into())?;
    let result = engine.call_fn::<i32>(&mut scope, &ast, "square", (5,))?; // Passing 5 as argument
    println!("Result: {}", result);

    Ok(())
}
