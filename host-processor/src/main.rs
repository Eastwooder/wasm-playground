use wasmtime::component::__internal::anyhow::Context;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Store};

wasmtime::component::bindgen!({
    world: "task",
    path: "../wit",
});

struct Processor {
    prefix: String,
}

impl TaskImports for Processor {
    fn log(&mut self, msg: String) {
        println!("{prefix}: {msg}", prefix = self.prefix);
    }
}

fn main() -> wasmtime::Result<()> {
    let file = "target/wasm32-wasip2/debug/guest_logic.wasm";
    let engine = Engine::default();
    let component = Component::from_file(&engine, file)?;
    let mut linker = Linker::new(&engine);
    Task::add_to_linker(&mut linker, |state: &mut Processor| state)?;
    let mut store = Store::new(
        &engine,
        Processor {
            prefix: "[info]".to_owned(),
        },
    );
    let bindings = Task::instantiate(&mut store, &component, &linker)?;
    bindings
        .call_run(&mut store)
        .context("trying to call 'run'")?;
    Ok(())
}
