use anyhow::{Context, Result};
use shared_types::OutputEvent;
use std::path::Path;
use wasmtime::*;

pub fn run_wasm_plugin<P: AsRef<Path>>(wasm_path: P, event: &OutputEvent) -> Result<bool> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, &wasm_path)
        .with_context(|| format!("Failed to load WASM module at {:?}", wasm_path.as_ref()))?;

    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    let instance = linker.instantiate(&mut store, &module)?;

    let filter_func = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "filter_event")
        .context("Failed to find 'filter_event' function in WASM module")?;

    let json_bytes = serde_json::to_vec(event)?;
    let len = json_bytes.len() as i32;

    let result = filter_func.call(&mut store, (0, len))?;

    Ok(result == 1)
}
