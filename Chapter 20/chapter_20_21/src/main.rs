use rhai::{plugin::*, Engine};

#[export_module]
pub(crate) mod my_plugin {
    pub fn module() -> rhai::Shared<rhai::Module> {
        rhai::exported_module!(my_plugin).into()
    }
    pub fn hello_world() -> String {
        "Hello, world!".to_string()
    }
}

fn main() {
    let mut engine = Engine::new();
    engine.register_global_module(my_plugin::module().into());
}
