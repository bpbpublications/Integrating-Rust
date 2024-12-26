use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    // Load and execute the Rhai script file
    engine.eval_file(("/path/to/example.rhai").into())?;

    Ok(())
}
